// Native Qt 6 Widgets frontend. The Rust engine owns all application operations.
// Build: c++ -std=c++17 -fPIC native/qt.cpp $(pkg-config --cflags --libs Qt6Widgets) -o frontend-qt
#include "i18n_fallback.h"
#include <QApplication>
#include <QAbstractButton>
#include <QLibraryInfo>
#include <QTranslator>
#include <QCloseEvent>
#include <QComboBox>
#include <QDesktopServices>
#include <QDialog>
#include <QDialogButtonBox>
#include <QDir>
#include <QFileDialog>
#include <QFileInfo>
#include <QFont>
#include <QFormLayout>
#include <QFrame>
#include <QGuiApplication>
#include <QHBoxLayout>
#include <QHeaderView>
#include <QIcon>
#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>
#include <QLabel>
#include <QLineEdit>
#include <QListWidget>
#include <QMainWindow>
#include <QMenuBar>
#include <QMessageBox>
#include <QPalette>
#include <QPlainTextEdit>
#include <QPointer>
#include <QProcess>
#include <QProgressBar>
#include <QPushButton>
#include <QRegularExpression>
#include <QResizeEvent>
#include <QScreen>
#include <QScrollArea>
#include <QSplitter>
#include <QStackedWidget>
#include <QStandardPaths>
#include <QStatusBar>
#include <QStyle>
#include <QStyleFactory>
#include <QTimer>
#include <QToolBar>
#include <QToolButton>
#include <QUrl>
#include <QVBoxLayout>
#include <deque>
#include <functional>
#include <memory>

namespace {
// English is embedded for the initial frame and transport failures. The Rust
// scan response replaces it with English + the selected locale's catalog.
static QJsonObject catalog = QJsonDocument::fromJson(QByteArray(APPIMAGE_EN_JSON)).object();
static QHash<QString, QString> sourceMessages;
static QTranslator qtStandardTranslator;
QString T(const char *source) {
    const auto key = QString::fromUtf8(source);
    const auto translated = catalog.value(key).toString(key);
    sourceMessages.insert(key, key);
    sourceMessages.insert(translated, key);
    return translated;
}
QString translatedProperty(QObject *object, const char *property, const QString &current) {
    const auto name = QByteArray("i18n-") + property;
    QString key = object->property(name.constData()).toString();
    if (key.isEmpty()) key = sourceMessages.value(current);
    if (key.isEmpty()) return current;
    object->setProperty(name.constData(), key);
    return catalog.value(key).toString(key);
}
void retranslateTree(QObject *root) {
    auto objects = root->findChildren<QObject *>(); objects.prepend(root);
    for (auto *object : objects) {
        if (auto *label = qobject_cast<QLabel *>(object)) label->setText(translatedProperty(label, "text", label->text()));
        else if (auto *button = qobject_cast<QAbstractButton *>(object)) button->setText(translatedProperty(button, "text", button->text()));
        else if (auto *action = qobject_cast<QAction *>(object)) action->setText(translatedProperty(action, "text", action->text()));
        else if (auto *entry = qobject_cast<QLineEdit *>(object)) entry->setPlaceholderText(translatedProperty(entry, "placeholder", entry->placeholderText()));
        else if (auto *text = qobject_cast<QPlainTextEdit *>(object)) text->setPlaceholderText(translatedProperty(text, "placeholder", text->placeholderText()));
        if (auto *widget = qobject_cast<QWidget *>(object)) {
            widget->setWindowTitle(translatedProperty(widget, "windowTitle", widget->windowTitle()));
            widget->setToolTip(translatedProperty(widget, "tooltip", widget->toolTip()));
        }
    }
}
bool confirmDialog(QWidget *parent, const QString &title, const QString &message, bool acceptDefault = false) {
    QMessageBox box(QMessageBox::Question, title, message, QMessageBox::NoButton, parent);
    auto *accept = box.addButton(T("确认"), QMessageBox::AcceptRole);
    auto *cancel = box.addButton(T("取消"), QMessageBox::RejectRole);
    box.setDefaultButton(acceptDefault ? accept : cancel); box.exec();
    return box.clickedButton() == accept;
}
using Reply = std::function<void(bool, const QJsonValue &, const QString &)>;

// Serialize requests so saves, scans and filesystem operations cannot race.
class EngineClient final : public QObject {
public:
    EngineClient(QString engine, QObject *parent) : QObject(parent), engine_(std::move(engine)) {}
    void request(QJsonObject payload, Reply callback) {
        queue_.push_back({std::move(payload), std::move(callback)});
        startNext();
    }
    QString engine() const { return engine_; }
private:
    struct Request { QJsonObject payload; Reply callback; };
    QString engine_;
    std::deque<Request> queue_;
    bool running_ = false;
    void startNext() {
        if (running_ || queue_.empty()) return;
        running_ = true;
        auto request = std::make_shared<Request>(std::move(queue_.front()));
        queue_.pop_front();
        auto *process = new QProcess(this);
        auto output = std::make_shared<QByteArray>();
        auto done = std::make_shared<bool>(false);
        auto complete = [this, process, request, output, done](bool transportOk, QString problem) {
            if (*done) return;
            *done = true;
            output->append(process->readAllStandardOutput());
            bool ok = false;
            QJsonValue data;
            if (transportOk && output->size() <= 8 * 1024 * 1024) {
                QJsonParseError parseError;
                const auto document = QJsonDocument::fromJson(*output, &parseError);
                if (parseError.error == QJsonParseError::NoError && document.isObject()) {
                    const auto reply = document.object();
                    ok = reply.value("ok").toBool(false);
                    data = reply.value("data");
                    if (!ok) problem = reply.value("error").toString(QStringLiteral("Operation failed"));
                } else {
                    problem = QStringLiteral("The engine returned invalid JSON: %1").arg(parseError.errorString());
                }
            } else if (problem.isEmpty()) {
                problem = process->property("rpc_error").toString();
                if (problem.isEmpty()) problem = QString::fromUtf8(process->readAllStandardError()).right(2000).trimmed();
                if (problem.isEmpty()) problem = QStringLiteral("The engine process did not complete successfully");
            }
            running_ = false;
            process->deleteLater();
            request->callback(ok, data, problem);
            startNext();
        };
        connect(process, &QProcess::readyReadStandardOutput, this, [process, output] {
            output->append(process->readAllStandardOutput());
            if (output->size() > 8 * 1024 * 1024) {
                process->setProperty("rpc_error", QStringLiteral("The engine response exceeded the size limit"));
                process->kill();
            }
        });
        connect(process, &QProcess::finished, this, [complete](int code, QProcess::ExitStatus status) {
            complete(code == 0 && status == QProcess::NormalExit, {});
        });
        connect(process, &QProcess::errorOccurred, this, [process, complete](QProcess::ProcessError error) {
            if (error == QProcess::FailedToStart) complete(false, QStringLiteral("Could not start the Rust engine: %1").arg(process->errorString()));
        });
        QTimer::singleShot(360000, process, [process, done] {
            if (!*done && process->state() != QProcess::NotRunning) {
                process->setProperty("rpc_error", QStringLiteral("Operation timed out; check the network connection and retry"));
                process->kill();
            }
        });
        process->setProgram(engine_);
        process->setArguments({QStringLiteral("--rpc"), QString::fromUtf8(QJsonDocument(request->payload).toJson(QJsonDocument::Compact))});
        process->start();
    }
};

QString byteSize(double bytes) {
    if (bytes >= 1024.0 * 1024 * 1024) return QStringLiteral("%1 GiB").arg(bytes / (1024 * 1024 * 1024), 0, 'f', 2);
    if (bytes >= 1024 * 1024) return QStringLiteral("%1 MiB").arg(bytes / (1024 * 1024), 0, 'f', 1);
    if (bytes >= 1024) return QStringLiteral("%1 KiB").arg(bytes / 1024, 0, 'f', 1);
    return QStringLiteral("%1 B").arg(bytes, 0, 'f', 0);
}
QLabel *plainLabel(const QString &text = {}) {
    auto *label = new QLabel(text);
    label->setTextFormat(Qt::PlainText);
    label->setWordWrap(true);
    label->setTextInteractionFlags(Qt::TextSelectableByMouse);
    return label;
}

class ManagerWindow final : public QMainWindow {
public:
    explicit ManagerWindow(const QString &engine) : client_(engine, this) {
        setWindowTitle(QStringLiteral("AppimageMannger · Qt"));
        setWindowIcon(QIcon::fromTheme(QStringLiteral("application-x-executable")));
        setMinimumSize(920, 620);
        resize(1180, 780);
        buildMenus();
        buildLayout();
        saveTimer_.setSingleShot(true);
        saveTimer_.setInterval(300);
        connect(&saveTimer_, &QTimer::timeout, this, [this] { saveConfig(); });
        geometryTimer_.setSingleShot(true);
        geometryTimer_.setInterval(600);
        connect(&geometryTimer_, &QTimer::timeout, this, [this] { rememberGeometry(); saveConfig(); });
        statusBar()->showMessage(T("正在读取应用目录…"));
        scan();
    }
protected:
    void resizeEvent(QResizeEvent *event) override {
        QMainWindow::resizeEvent(event);
        if (ready_ && !closing_) geometryTimer_.start();
    }
    void closeEvent(QCloseEvent *event) override {
        if (!ready_ || closing_) { event->accept(); return; }
        event->ignore();
        if (closePending_) return;
        closePending_ = true;
        setEnabled(false);
        saveTimer_.stop();
        geometryTimer_.stop();
        rememberGeometry();
        statusBar()->showMessage(T("正在保存窗口状态…"));
        saveConfig([this](bool ok) {
            if (!ok) { closePending_ = false; setEnabled(true); return; }
            closing_ = true;
            close();
        });
    }
private:
    EngineClient client_;
    QJsonObject config_;
    QJsonArray apps_;
    QHash<QString, QJsonObject> checks_;
    bool ready_ = false, busy_ = false, closing_ = false, closePending_ = false;
    quint64 configRevision_ = 0, persistedRevision_ = 0;
    QString version_, selectedPath_;
    QJsonArray locales_;
    QTimer saveTimer_, geometryTimer_;
    QListWidget *list_ = nullptr;
    QLineEdit *search_ = nullptr, *repository_ = nullptr;
    QLabel *count_ = nullptr, *directory_ = nullptr, *name_ = nullptr, *summary_ = nullptr, *icon_ = nullptr;
    QLabel *versionLabel_ = nullptr, *sizeLabel_ = nullptr, *architecture_ = nullptr, *path_ = nullptr, *updateStatus_ = nullptr;
    QPlainTextEdit *notes_ = nullptr;
    QStackedWidget *details_ = nullptr;
    QProgressBar *progress_ = nullptr;
    QPushButton *launch_ = nullptr, *uninstall_ = nullptr, *check_ = nullptr, *install_ = nullptr, *release_ = nullptr, *sourceSave_ = nullptr, *integrate_ = nullptr;
    QAction *importAction_ = nullptr, *refreshAction_ = nullptr;

    QJsonObject selectedApp() const {
        for (const auto &value : apps_) if (value.toObject().value("path").toString() == selectedPath_) return value.toObject();
        return {};
    }
    void error(const QString &message) {
        statusBar()->showMessage(message, 15000);
        QMessageBox box(QMessageBox::Warning, QStringLiteral("Error"), message, QMessageBox::NoButton, this);
        box.addButton(T("关闭"), QMessageBox::AcceptRole); box.exec();
    }
    void changed() { ++configRevision_; if (ready_ && !closing_) saveTimer_.start(); }
    void saveConfig(std::function<void(bool)> after = {}) {
        if (!ready_) { if (after) after(false); return; }
        saveTimer_.stop();
        const auto snapshot = config_;
        const auto revision = configRevision_;
        client_.request({{"action", "save_config"}, {"config", snapshot}}, [this, revision, after](bool ok, const QJsonValue &data, const QString &problem) {
            if (!ok) error(T("设置保存失败：%1").arg(problem));
            else {
                persistedRevision_ = qMax(persistedRevision_, revision);
                if (revision == configRevision_ && data.isObject()) config_ = data.toObject();
            }
            if (after) after(ok);
        });
    }
    void rememberGeometry() {
        config_["maximized"] = isMaximized();
        if (!isMaximized()) { config_["window_width"] = width(); config_["window_height"] = height(); }
        ++configRevision_;
    }
    void restoreGeometry() {
        const auto available = QGuiApplication::primaryScreen()->availableGeometry();
        const int width = qBound(920, config_.value("window_width").toInt(1180), qMax(920, available.width()));
        const int height = qBound(620, config_.value("window_height").toInt(780), qMax(620, available.height()));
        resize(width, height);
        if (config_.value("maximized").toBool(false)) showMaximized();
    }
    void setBusy(bool busy, const QString &message = {}) {
        busy_ = busy;
        progress_->setVisible(busy);
        importAction_->setEnabled(!busy);
        refreshAction_->setEnabled(!busy);
        refreshButtons();
        if (!message.isEmpty()) statusBar()->showMessage(message);
    }
    void refreshButtons() {
        const bool exists = !selectedApp().isEmpty();
        for (auto *button : {launch_, uninstall_, check_, sourceSave_, integrate_}) if (button) button->setEnabled(exists && !busy_);
        const auto asset = checks_.value(selectedPath_).value("asset");
        install_->setEnabled(exists && !busy_ && asset.isObject() && !asset.toObject().isEmpty());
        release_->setEnabled(exists && !checks_.value(selectedPath_).value("release_url").toString().isEmpty());
    }
    void buildMenus() {
        auto *file = menuBar()->addMenu(T("文件"));
        importAction_ = file->addAction(style()->standardIcon(QStyle::SP_DialogOpenButton), T("导入 AppImage…"));
        importAction_->setShortcut(QKeySequence::Open);
        connect(importAction_, &QAction::triggered, this, [this] { importImage(); });
        file->addAction(T("打开应用目录"), this, [this] { openDirectory(); });
        file->addSeparator();
        auto *quit = file->addAction(T("退出"), this, [this] { close(); });
        quit->setShortcut(QKeySequence::Quit);
        auto *tools = menuBar()->addMenu(T("工具"));
        refreshAction_ = tools->addAction(style()->standardIcon(QStyle::SP_BrowserReload), T("刷新应用列表"));
        refreshAction_->setShortcut(QKeySequence::Refresh);
        connect(refreshAction_, &QAction::triggered, this, [this] { scan(); });
        auto *settings = tools->addAction(style()->standardIcon(QStyle::SP_FileDialogDetailedView), T("设置…"));
        settings->setShortcut(QKeySequence::Preferences);
        connect(settings, &QAction::triggered, this, [this] { settingsDialog(); });
        auto *help = menuBar()->addMenu(T("帮助"));
        auto *about = help->addAction(T("关于 AppimageMannger…"));
        connect(about, &QAction::triggered, this, [this] {
            QMessageBox::about(this, T("关于 AppimageMannger"),
                T("<h3>AppimageMannger %1</h3><p>真实 Qt 6 Widgets 前端 · Rust 管理引擎</p><p>管理 AppImage 应用、目录、桌面入口和更新。</p><p>作者：xingwangzhe<br>AGPL-3.0<br>Qt 运行版本：%2</p>")
                .arg(version_.toHtmlEscaped(), QString::fromLatin1(qVersion())));
        });
        help->addAction(T("项目主页"), this, [] { QDesktopServices::openUrl(QUrl(QStringLiteral("https://github.com/xingwangzhe/AppimageMannger"))); });
        auto *toolbar = addToolBar(T("应用工具栏"));
        toolbar->setMovable(false);
        toolbar->setToolButtonStyle(Qt::ToolButtonTextBesideIcon);
        toolbar->addAction(importAction_);
        toolbar->addAction(refreshAction_);
        toolbar->addSeparator();
        toolbar->addAction(settings);
        toolbar->addAction(about);
    }
    void buildLayout() {
        auto *splitter = new QSplitter(Qt::Horizontal, this);
        splitter->setChildrenCollapsible(false);
        auto *sidebar = new QWidget;
        sidebar->setMinimumWidth(255);
        auto *left = new QVBoxLayout(sidebar);
        left->setContentsMargins(16, 18, 16, 12);
        left->setSpacing(12);
        auto *heading = plainLabel(T("应用程序"));
        auto font = heading->font(); font.setPointSize(15); font.setBold(true); heading->setFont(font);
        count_ = plainLabel(T("正在扫描…"));
        search_ = new QLineEdit;
        search_->setPlaceholderText(T("搜索名称或版本"));
        search_->setClearButtonEnabled(true);
        list_ = new QListWidget;
        list_->setIconSize(QSize(42, 42));
        list_->setSpacing(4);
        list_->setUniformItemSizes(true);
        directory_ = plainLabel();
        auto small = directory_->font(); small.setPointSize(qMax(8, small.pointSize() - 1)); directory_->setFont(small);
        left->addWidget(heading); left->addWidget(count_); left->addWidget(search_); left->addWidget(list_, 1); left->addWidget(directory_);
        connect(search_, &QLineEdit::textChanged, this, [this](const QString &query) {
            for (int i = 0; i < list_->count(); ++i) list_->item(i)->setHidden(!list_->item(i)->text().contains(query, Qt::CaseInsensitive));
        });
        connect(list_, &QListWidget::currentItemChanged, this, [this](QListWidgetItem *item, QListWidgetItem *) {
            if (!item) return;
            selectedPath_ = item->data(Qt::UserRole).toString();
            config_["last_selected_path"] = selectedPath_;
            changed();
            showDetails();
        });
        connect(list_, &QListWidget::itemDoubleClicked, this, [this](QListWidgetItem *) { runAction("launch", T("正在启动应用…")); });
        splitter->addWidget(sidebar);
        details_ = new QStackedWidget;
        auto *empty = new QWidget;
        auto *emptyLayout = new QVBoxLayout(empty);
        emptyLayout->addStretch();
        auto *emptyTitle = plainLabel(T("让 AppImage 各归其位"));
        emptyTitle->setAlignment(Qt::AlignCenter);
        auto titleFont = emptyTitle->font(); titleFont.setPointSize(23); titleFont.setBold(true); emptyTitle->setFont(titleFont);
        auto *emptyText = plainLabel(T("从左侧选择应用查看详情，或导入一个 AppImage 开始管理。")); emptyText->setAlignment(Qt::AlignCenter);
        auto *emptyImport = new QPushButton(T("导入 AppImage…"));
        connect(emptyImport, &QPushButton::clicked, this, [this] { importImage(); });
        emptyLayout->addWidget(emptyTitle); emptyLayout->addWidget(emptyText); emptyLayout->addWidget(emptyImport, 0, Qt::AlignCenter); emptyLayout->addStretch();
        details_->addWidget(empty);
        auto *scroll = new QScrollArea;
        scroll->setFrameShape(QFrame::NoFrame);
        scroll->setWidgetResizable(true);
        auto *body = new QWidget;
        auto *right = new QVBoxLayout(body);
        right->setContentsMargins(28, 28, 28, 24); right->setSpacing(18);
        auto *hero = new QHBoxLayout;
        icon_ = new QLabel; icon_->setFixedSize(80, 80); icon_->setAlignment(Qt::AlignCenter);
        auto *titles = new QVBoxLayout;
        name_ = plainLabel(); auto nameFont = name_->font(); nameFont.setPointSize(24); nameFont.setBold(true); name_->setFont(nameFont);
        summary_ = plainLabel();
        titles->addWidget(name_); titles->addWidget(summary_); hero->addWidget(icon_); hero->addSpacing(10); hero->addLayout(titles, 1); right->addLayout(hero);
        auto *actions = new QHBoxLayout;
        launch_ = new QPushButton(style()->standardIcon(QStyle::SP_MediaPlay), T("启动应用"));
        auto *folder = new QPushButton(style()->standardIcon(QStyle::SP_DirOpenIcon), T("打开目录"));
        integrate_ = new QPushButton(T("加入应用菜单"));
        uninstall_ = new QPushButton(style()->standardIcon(QStyle::SP_TrashIcon), T("卸载…"));
        for (auto *button : {launch_, folder, integrate_, uninstall_}) actions->addWidget(button);
        actions->addStretch(); right->addLayout(actions);
        connect(launch_, &QPushButton::clicked, this, [this] { runAction("launch", T("正在启动应用…")); });
        connect(folder, &QPushButton::clicked, this, [this] { openDirectory(); });
        connect(integrate_, &QPushButton::clicked, this, [this] { runAction("integrate", T("正在创建应用菜单入口…"), true); });
        connect(uninstall_, &QPushButton::clicked, this, [this] {
            const auto app = selectedApp(); if (app.isEmpty()) return;
            if (confirmDialog(this, T("卸载应用"), T("将“%1”移入回收站？\n\n应用文件及对应桌面入口可以从回收站恢复。个人配置保持不变。").arg(app.value("name").toString()), false))
                runAction("uninstall", T("正在将应用移入回收站…"), true);
        });
        auto *separator = new QFrame; separator->setFrameShape(QFrame::HLine); right->addWidget(separator);
        auto *form = new QFormLayout;
        form->setVerticalSpacing(14);
        versionLabel_ = plainLabel(); sizeLabel_ = plainLabel(); architecture_ = plainLabel(); path_ = plainLabel();
        form->addRow(T("当前版本"), versionLabel_); form->addRow(T("安装大小"), sizeLabel_); form->addRow(T("架构"), architecture_); form->addRow(T("安装位置"), path_); right->addLayout(form);
        auto *sourceTitle = plainLabel(T("更新源")); auto bold = sourceTitle->font(); bold.setBold(true); sourceTitle->setFont(bold); right->addWidget(sourceTitle);
        auto *sourceRow = new QHBoxLayout;
        repository_ = new QLineEdit; repository_->setPlaceholderText(T("owner/repository 或 GitHub 仓库 HTTPS 地址")); repository_->setClearButtonEnabled(true);
        sourceSave_ = new QPushButton(T("保存更新源")); sourceRow->addWidget(repository_, 1); sourceRow->addWidget(sourceSave_); right->addLayout(sourceRow);
        auto *sourceHint = plainLabel(T("优先识别 AppImage 内的更新信息；未包含时可填写对应的 GitHub 仓库。")); right->addWidget(sourceHint);
        connect(repository_, &QLineEdit::textEdited, this, [this](const QString &text) {
            if (selectedPath_.isEmpty()) return;
            auto drafts = config_.value("repository_drafts").toObject(); drafts[QFileInfo(selectedPath_).fileName()] = text; config_["repository_drafts"] = drafts; changed();
        });
        connect(sourceSave_, &QPushButton::clicked, this, [this] { saveRepository(); });
        connect(repository_, &QLineEdit::returnPressed, this, [this] { saveRepository(); });
        auto *updateRow = new QHBoxLayout;
        check_ = new QPushButton(style()->standardIcon(QStyle::SP_BrowserReload), T("检查更新"));
        install_ = new QPushButton(style()->standardIcon(QStyle::SP_ArrowDown), T("安装更新"));
        release_ = new QPushButton(T("查看发布页面"));
        updateRow->addWidget(check_); updateRow->addWidget(install_); updateRow->addWidget(release_); updateRow->addStretch(); right->addLayout(updateRow);
        updateStatus_ = plainLabel(T("尚未检查更新")); right->addWidget(updateStatus_);
        notes_ = new QPlainTextEdit; notes_->setReadOnly(true); notes_->setPlaceholderText(T("发布说明将在检查更新后显示")); notes_->setMinimumHeight(125); right->addWidget(notes_);
        connect(check_, &QPushButton::clicked, this, [this] { checkUpdate(); });
        connect(install_, &QPushButton::clicked, this, [this] { installUpdate(); });
        connect(release_, &QPushButton::clicked, this, [this] {
            const auto url = QUrl(checks_.value(selectedPath_).value("release_url").toString());
            if (url.scheme() == "https") QDesktopServices::openUrl(url);
        });
        right->addStretch(); scroll->setWidget(body); details_->addWidget(scroll);
        splitter->addWidget(details_); splitter->setStretchFactor(0, 0); splitter->setStretchFactor(1, 1); splitter->setSizes({300, 880}); setCentralWidget(splitter);
        progress_ = new QProgressBar; progress_->setRange(0, 0); progress_->setFixedWidth(130); progress_->setMaximumHeight(13); progress_->hide(); statusBar()->addPermanentWidget(progress_);
        refreshButtons();
    }
    void applyAppearance() {
        const bool dark = config_.value("appearance").toString() == "dark";
        QPalette palette = QApplication::style()->standardPalette();
        if (dark) {
            palette.setColor(QPalette::Window, QColor(37, 40, 47));
            palette.setColor(QPalette::WindowText, QColor(238, 240, 245));
            palette.setColor(QPalette::Base, QColor(28, 31, 37));
            palette.setColor(QPalette::AlternateBase, QColor(43, 47, 55));
            palette.setColor(QPalette::Text, QColor(238, 240, 245));
            palette.setColor(QPalette::Button, QColor(48, 52, 61));
            palette.setColor(QPalette::ButtonText, QColor(238, 240, 245));
            palette.setColor(QPalette::ToolTipBase, QColor(48, 52, 61));
            palette.setColor(QPalette::ToolTipText, QColor(238, 240, 245));
            palette.setColor(QPalette::Highlight, QColor(79, 113, 223));
            palette.setColor(QPalette::HighlightedText, Qt::white);
            palette.setColor(QPalette::PlaceholderText, QColor(151, 158, 175));
            palette.setColor(QPalette::Link, QColor(139, 173, 255));
            palette.setColor(QPalette::Disabled, QPalette::Text, QColor(125, 131, 145));
            palette.setColor(QPalette::Disabled, QPalette::ButtonText, QColor(125, 131, 145));
        }
        qApp->setPalette(palette);
    }
    void scan(const QString &prefer = {}) {
        if (busy_) return;
        setBusy(true, T("正在扫描 AppImage 应用…"));
        const QString selection = prefer.isEmpty() ? selectedPath_ : prefer;
        client_.request({{"action", "scan"}}, [this, selection](bool ok, const QJsonValue &data, const QString &problem) {
            setBusy(false);
            if (!ok) { error(problem); return; }
            const auto result = data.toObject();
            const bool first = !ready_;
            // Pending local edits remain authoritative until their queued save completes.
            if (first || persistedRevision_ == configRevision_) config_ = result.value("config").toObject();
            apps_ = result.value("apps").toArray(); version_ = result.value("version").toString();
            locales_ = result.value("locales").toArray();
            if (first) {
                const auto translations = result.value("translations").toObject();
                for (auto it = translations.begin(); it != translations.end(); ++it) catalog.insert(it.key(), it.value());
                const auto locale = result.value("resolved_language").toString(QStringLiteral("en"));
                const bool rtl = locale.startsWith("ar") || locale.startsWith("he") || locale.startsWith("fa") || locale.startsWith("ur");
                QApplication::setLayoutDirection(rtl ? Qt::RightToLeft : Qt::LeftToRight);
                auto qtLocale = locale; qtLocale.replace('-', '_');
                if (qtStandardTranslator.load("qtbase_" + qtLocale, QLibraryInfo::path(QLibraryInfo::TranslationsPath))) qApp->installTranslator(&qtStandardTranslator);
                retranslateTree(this);
            }
            if (first) { applyAppearance(); restoreGeometry(); show(); }
            ready_ = true;
            directory_->setText(config_.value("applications_dir").toString());
            count_->setText(T("%1 个 AppImage").arg(apps_.size()));
            const QString target = selection.isEmpty() ? config_.value("last_selected_path").toString() : selection;
            const QSignalBlocker blocker(list_);
            list_->clear();
            QListWidgetItem *selected = nullptr;
            for (const auto &value : apps_) {
                const auto app = value.toObject();
                QIcon icon(app.value("icon_path").toString()); if (icon.isNull()) icon = style()->standardIcon(QStyle::SP_DesktopIcon);
                auto *item = new QListWidgetItem(icon, app.value("name").toString() + "\n" + app.value("version").toString(), list_);
                item->setData(Qt::UserRole, app.value("path")); item->setToolTip(app.value("path").toString()); item->setSizeHint(QSize(230, 66));
                item->setHidden(!item->text().contains(search_->text(), Qt::CaseInsensitive));
                if (app.value("path").toString() == target) selected = item;
            }
            if (!selected && list_->count()) selected = list_->item(0);
            if (selected) { list_->setCurrentItem(selected); selectedPath_ = selected->data(Qt::UserRole).toString(); }
            else selectedPath_.clear();
            showDetails();
            statusBar()->showMessage(T("已读取 %1 个应用 · Qt %2").arg(apps_.size()).arg(QString::fromLatin1(qVersion())), 8000);
        });
    }
    void showDetails() {
        const auto app = selectedApp();
        if (app.isEmpty()) { details_->setCurrentIndex(0); refreshButtons(); return; }
        details_->setCurrentIndex(1);
        name_->setText(app.value("name").toString()); summary_->setText(app.value("description").toString());
        QIcon icon(app.value("icon_path").toString()); if (icon.isNull()) icon = style()->standardIcon(QStyle::SP_DesktopIcon);
        icon_->setPixmap(icon.pixmap(QSize(76, 76)));
        versionLabel_->setText(app.value("version").toString()); sizeLabel_->setText(byteSize(app.value("size_bytes").toDouble())); architecture_->setText(app.value("architecture").toString()); path_->setText(app.value("path").toString());
        const auto drafts = config_.value("repository_drafts").toObject();
        repository_->setText(drafts.contains(QFileInfo(selectedPath_).fileName()) ? drafts.value(QFileInfo(selectedPath_).fileName()).toString() : app.value("repository").toString());
        const auto check = checks_.value(selectedPath_);
        updateStatus_->setText(check.isEmpty() ? T("尚未检查更新") : catalog.value(check.value("status").toString()).toString(check.value("status").toString()) + (check.value("latest_version").toString().isEmpty() ? "" : T(" · 最新版本 %1").arg(check.value("latest_version").toString())));
        notes_->setPlainText(check.value("notes").toString());
        refreshButtons();
    }
    void importImage() {
        if (busy_) return;
        const auto path = QFileDialog::getOpenFileName(this, T("导入 AppImage"), QDir::homePath(), T("AppImage 应用 (*.AppImage *.appimage);;所有文件 (*)"));
        if (path.isEmpty()) return;
        setBusy(true, T("正在导入应用…"));
        client_.request({{"action", "import"}, {"path", path}}, [this](bool ok, const QJsonValue &data, const QString &problem) {
            setBusy(false); if (!ok) { error(problem); return; } scan(data.toObject().value("path").toString());
        });
    }
    void runAction(const QString &action, const QString &message, bool refresh = false) {
        if (busy_ || selectedPath_.isEmpty()) return;
        const auto path = selectedPath_;
        setBusy(true, message);
        client_.request({{"action", action}, {"path", path}}, [this, action, refresh](bool ok, const QJsonValue &, const QString &problem) {
            setBusy(false); if (!ok) { error(problem); return; }
            if (refresh) scan(); else statusBar()->showMessage(action == "launch" ? T("已发送应用启动请求") : T("操作完成"), 6000);
        });
    }
    void openDirectory() {
        client_.request({{"action", "open_directory"}}, [this](bool ok, const QJsonValue &, const QString &problem) { if (!ok) error(problem); });
    }
    void saveRepository() {
        const auto app = selectedApp(); if (app.isEmpty() || busy_) return;
        QString repo = repository_->text().trimmed();
        if (repo.startsWith("https://github.com/")) repo.remove(0, 19);
        while (repo.endsWith('/')) repo.chop(1);
        if (repo.endsWith(".git")) repo.chop(4);
        static const QRegularExpression pattern(QStringLiteral("^[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+$"));
        if (!repo.isEmpty() && (!pattern.match(repo).hasMatch() || repo.startsWith("../") || repo.endsWith("/.."))) { error(T("请填写 owner/repository 或 GitHub 仓库的 HTTPS 地址")); return; }
        auto repositories = config_.value("repositories").toObject();
        const auto key = QFileInfo(selectedPath_).fileName();
        if (repo.isEmpty()) repositories.remove(key); else repositories[key] = repo;
        config_["repositories"] = repositories;
        auto drafts = config_.value("repository_drafts").toObject(); drafts.remove(selectedPath_); drafts.remove(key); config_["repository_drafts"] = drafts;
        ++configRevision_;
        setBusy(true, T("正在保存更新源…"));
        saveConfig([this](bool ok) { setBusy(false); if (ok) { checks_.remove(selectedPath_); scan(); } });
    }
    void checkUpdate() {
        if (busy_ || selectedPath_.isEmpty()) return;
        const auto path = selectedPath_;
        setBusy(true, T("正在检查最新稳定版本…"));
        client_.request({{"action", "check_update"}, {"path", path}}, [this, path](bool ok, const QJsonValue &data, const QString &problem) {
            setBusy(false); if (!ok) { error(problem); return; }
            checks_[path] = data.toObject(); if (selectedPath_ == path) showDetails();
            statusBar()->showMessage(catalog.value(data.toObject().value("status").toString()).toString(data.toObject().value("status").toString()), 12000);
        });
    }
    void installUpdate() {
        const auto app = selectedApp(); const auto asset = checks_.value(selectedPath_).value("asset");
        if (busy_ || app.isEmpty() || !asset.isObject()) return;
        if (confirmDialog(this, T("安装更新"), T("为“%1”安装版本 %2？\n\n下载和校验完成后替换应用，保留原版本备份。").arg(app.value("name").toString(), asset.toObject().value("version").toString()), true) == false) return;
        const auto path = selectedPath_; setBusy(true, T("正在下载、验证并安装更新…"));
        client_.request({{"action", "install_update"}, {"path", path}, {"asset", asset}}, [this, path](bool ok, const QJsonValue &, const QString &problem) {
            setBusy(false); if (!ok) { error(problem); return; } checks_.remove(path); scan(path);
        });
    }
    void settingsDialog() {
        if (!ready_) return;
        auto *dialog = new QDialog(this); dialog->setAttribute(Qt::WA_DeleteOnClose); dialog->setWindowTitle(T("设置")); dialog->setMinimumWidth(590);
        auto *layout = new QVBoxLayout(dialog);
        auto *heading = plainLabel(T("应用与界面")); auto font = heading->font(); font.setPointSize(18); font.setBold(true); heading->setFont(font); layout->addWidget(heading);
        auto *form = new QFormLayout;
        auto *directoryRow = new QWidget; auto *directoryLayout = new QHBoxLayout(directoryRow); directoryLayout->setContentsMargins(0, 0, 0, 0);
        auto *directory = new QLineEdit;
        directory->setText(config_.value("directory_draft").isString() ? config_.value("directory_draft").toString() : config_.value("applications_dir").toString());
        auto *browse = new QPushButton(T("选择…")); directoryLayout->addWidget(directory, 1); directoryLayout->addWidget(browse);
        auto *appearance = new QComboBox; appearance->addItem(T("浅色"), "light"); appearance->addItem(T("深色"), "dark"); appearance->setCurrentIndex(config_.value("appearance").toString() == "dark" ? 1 : 0);
        auto *backend = new QComboBox; backend->addItem(T("GPUI / Ely 原生 GPU 界面"), "gpui"); backend->addItem(T("Qt 6 Widgets 原生界面"), "qt"); backend->addItem(T("GNOME / GTK 4 原生界面"), "gnome");
        const int current = backend->findData(config_.value("window_style").toString()); backend->setCurrentIndex(current < 0 ? 1 : current);
        form->addRow(T("AppImage 安装目录"), directoryRow); form->addRow(T("外观（立即保存）"), appearance); form->addRow(T("界面后端"), backend); layout->addLayout(form);
        auto *hint = plainLabel(T("目录设置保存后重新扫描。切换界面后端会保存设置并重新打开应用。现有 AppImage 不会自动搬移。")); layout->addWidget(hint);
        auto *language = new QComboBox;
        language->addItem(T("跟随系统"), QStringLiteral("system"));
        for (const auto &locale : locales_) { const auto item = locale.toObject(); language->addItem(item.value("name").toString(), item.value("code").toString()); }
        const int languageIndex = language->findData(config_.value("language").toString(QStringLiteral("system")));
        language->setCurrentIndex(languageIndex < 0 ? 0 : languageIndex);
        form->addRow(T("语言"), language);
        layout->addWidget(plainLabel(T("选择语言后会保存设置并重新打开应用。")));
        auto *buttons = new QDialogButtonBox(QDialogButtonBox::Save | QDialogButtonBox::Cancel);
        buttons->button(QDialogButtonBox::Save)->setText(T("保存")); buttons->button(QDialogButtonBox::Cancel)->setText(T("取消"));
        layout->addWidget(buttons);
        connect(language, &QComboBox::currentIndexChanged, dialog, [this, dialog, language](int) {
            const auto selected = language->currentData().toString();
            if (selected == config_.value("language").toString(QStringLiteral("system"))) return;
            const auto previous = config_; config_["language"] = selected; rememberGeometry(); ++configRevision_;
            dialog->setEnabled(false);
            const QPointer<QDialog> guard(dialog);
            saveConfig([this, guard, previous](bool ok) {
                if (!ok) { config_ = previous; ++configRevision_; if (guard) guard->setEnabled(true); return; }
                if (!QProcess::startDetached(client_.engine(), {})) { if (guard) guard->setEnabled(true); error(QStringLiteral("Settings were saved, but the application could not restart. Reopen AppimageMannger.")); return; }
                closing_ = true; qApp->quit();
            });
        });
        connect(browse, &QPushButton::clicked, dialog, [this, dialog, directory] { const auto chosen = QFileDialog::getExistingDirectory(dialog, T("选择 AppImage 目录"), directory->text()); if (!chosen.isEmpty()) directory->setText(chosen); });
        connect(directory, &QLineEdit::textChanged, dialog, [this](const QString &text) { config_["directory_draft"] = text; changed(); });
        connect(appearance, &QComboBox::currentIndexChanged, dialog, [this, appearance](int) { config_["appearance"] = appearance->currentData().toString(); changed(); applyAppearance(); });
        connect(buttons, &QDialogButtonBox::rejected, dialog, &QDialog::reject);
        connect(buttons, &QDialogButtonBox::accepted, dialog, [this, dialog, directory, backend, buttons] {
            QString path = directory->text().trimmed(); if (path == "~") path = QDir::homePath(); else if (path.startsWith("~/")) path = QDir::homePath() + path.mid(1);
            if (!QFileInfo(path).isAbsolute() || path.contains('\n') || path.contains('\r')) { QMessageBox::warning(dialog, QStringLiteral("Warning"), T("请输入有效的绝对目录路径")); return; }
            const auto previous = config_;
            const auto nextBackend = backend->currentData().toString();
            const bool restart = nextBackend != config_.value("window_style").toString();
            config_["applications_dir"] = QDir::cleanPath(path); config_["directory_draft"] = QJsonValue::Null; config_["window_style"] = nextBackend; rememberGeometry(); ++configRevision_;
            buttons->setEnabled(false);
            const QPointer<QDialog> dialogGuard(dialog);
            const QPointer<QDialogButtonBox> buttonsGuard(buttons);
            saveConfig([this, dialogGuard, buttonsGuard, previous, restart](bool ok) {
                if (!ok) { config_ = previous; ++configRevision_; if (buttonsGuard) buttonsGuard->setEnabled(true); return; }
                if (dialogGuard) dialogGuard->accept();
                if (restart) {
                    if (!QProcess::startDetached(client_.engine(), {})) { error(T("设置已保存，但无法重新启动。请重新打开 AppimageMannger。")); return; }
                    closing_ = true; qApp->quit();
                } else scan();
            });
        });
        dialog->open();
    }
};
} // namespace

int main(int argc, char **argv) {
    QApplication application(argc, argv);
    application.setApplicationName(QStringLiteral("AppimageMannger"));
    application.setApplicationDisplayName(QStringLiteral("AppimageMannger"));
    application.setOrganizationName(QStringLiteral("xingwangzhe"));
    application.setQuitOnLastWindowClosed(true);
    // Fusion is a real Qt widget style, with a consistent light/dark palette.
    if (auto *style = QStyleFactory::create(QStringLiteral("Fusion"))) application.setStyle(style);
    QString engine = qEnvironmentVariable("APPIMAGE_MAN_NGER_ENGINE");
    if (engine.isEmpty()) engine = QStandardPaths::findExecutable(QStringLiteral("appimage-mannger"));
    if (engine.isEmpty()) {
        QMessageBox::critical(nullptr, QStringLiteral("AppimageMannger"), QStringLiteral("The appimage-mannger Rust engine was not found. Reinstall the application."));
        return 1;
    }
    ManagerWindow window(engine);
    // Embedded English covers startup while scan loads the selected locale.
    window.show();
    return application.exec();
}
