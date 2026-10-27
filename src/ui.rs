use crate::i18n::tr;
use crate::{
    manager::{self, AppInfo, Appearance, Config, WindowStyle},
    update::{self, UpdateCheck},
};
use ely_core::{
    Assets,
    buttons::{Button, ButtonVariant, SegmentedControl},
    forms::{Choice, Input, InputEvent, PathInput, Select, TextInput},
    layout::{Card, CardHeader, SplitPane},
    lists::ListItem,
    menus::MenuBar,
    motion::ProgressBar,
    primitives::{FocusScope, Icon, IconName, Image},
    theme::{ActiveTheme, IconSize, Mode, Theme},
};
use gpui::{
    App, AppContext, Bounds, Context, Entity, FocusHandle, InteractiveElement, IntoElement,
    KeyBinding, Menu, MenuItem, ParentElement, Render, SharedString, Styled, TitlebarOptions,
    Window, WindowBounds, WindowOptions, actions, div, prelude::*, px, size,
};
use std::{path::PathBuf, process::Command};

actions!(
    appimage_mannger,
    [Refresh, Import, Settings, About, ToggleTheme, Quit]
);

#[derive(Clone, Copy, PartialEq)]
enum Page {
    Library,
    Settings,
    About,
}
enum JobResult {
    Apps(Vec<AppInfo>, String),
    Check(UpdateCheck),
    Message(String),
}
struct ManagerView {
    focus: FocusHandle,
    config: Config,
    apps: Vec<AppInfo>,
    selected: Option<usize>,
    search: Entity<TextInput>,
    directory: Entity<TextInput>,
    source: Entity<TextInput>,
    import_path: Entity<TextInput>,
    page: Page,
    busy: bool,
    status: String,
    error: bool,
    checked: Option<UpdateCheck>,
    confirm_uninstall: bool,
    progress: Option<update::DownloadProgress>,
}

impl ManagerView {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let loaded = Config::load();
        let (config, mut status, mut error) = match loaded {
            Ok(c) => (c, tr("准备就绪").to_owned(), false),
            Err(e) => (Config::default(), tr(format!("配置读取失败：{e:#}")), true),
        };
        let focus = cx.focus_handle();
        window.focus(&focus, cx);
        let search = cx.new(|cx| {
            TextInput::new(window, cx)
                .label(tr("搜索应用"))
                .placeholder(tr("搜索应用名称…"))
        });
        cx.subscribe(&search, |_, _, _: &InputEvent, cx| cx.notify())
            .detach();
        let directory = cx.new(|cx| {
            let mut input = TextInput::new(window, cx).label(tr("应用安装目录"));
            input.set_text(
                config
                    .directory_draft
                    .clone()
                    .unwrap_or_else(|| config.applications_dir.to_string_lossy().to_string()),
                cx,
            );
            input
        });
        let source = cx.new(|cx| {
            TextInput::new(window, cx)
                .label(tr("GitHub 更新仓库"))
                .placeholder("owner/repository")
        });
        let import_path = cx.new(|cx| {
            TextInput::new(window, cx)
                .label(tr("导入 AppImage"))
                .placeholder(tr("选择本地 .AppImage 文件"))
        });
        let apps = match manager::scan(&config) {
            Ok(apps) => apps,
            Err(e) => {
                status = tr(format!("扫描失败：{e:#}"));
                error = true;
                Vec::new()
            }
        };
        let mut view = Self {
            focus,
            config,
            apps,
            selected: None,
            search,
            directory,
            source,
            import_path,
            page: Page::Library,
            busy: false,
            status,
            error,
            checked: None,
            confirm_uninstall: false,
            progress: None,
        };
        if !view.apps.is_empty() {
            let at = view
                .config
                .last_selected_path
                .as_ref()
                .and_then(|p| view.apps.iter().position(|a| &a.path == p))
                .unwrap_or(0);
            view.select(at, cx);
        }
        cx.subscribe(&view.directory, |view, field, event: &InputEvent, cx| {
            if *event == InputEvent::Changed {
                let value = field.read(cx).text().to_owned();
                let applied = view.config.applications_dir.to_string_lossy();
                view.config.directory_draft = if value == applied { None } else { Some(value) };
                view.persist(cx);
            }
        })
        .detach();
        cx.subscribe(&view.source, |view, field, event: &InputEvent, cx| {
            if *event == InputEvent::Changed
                && let Some(app) = view.selected_app()
            {
                let key = app
                    .path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned();
                let value = field.read(cx).text().to_owned();
                if value == app.repository.unwrap_or_default() {
                    view.config.repository_drafts.remove(&key);
                } else {
                    view.config.repository_drafts.insert(key, value);
                }
                view.persist(cx);
            }
            if *event == InputEvent::Submit {
                view.save_repository(cx);
            }
        })
        .detach();
        cx.observe_window_bounds(window, |view, window, cx| {
            let maximized = window.is_maximized();
            let bounds = window.bounds().size;
            let width = f32::from(bounds.width).max(880.0);
            let height = f32::from(bounds.height).max(580.0);
            if view.config.maximized != maximized
                || (!maximized
                    && (view.config.window_width != width || view.config.window_height != height))
            {
                view.config.maximized = maximized;
                if !maximized {
                    view.config.window_width = width;
                    view.config.window_height = height;
                }
                view.persist(cx);
            }
        })
        .detach();
        let owner = cx.weak_entity();
        ely_core::shell::guard_close(window, cx, move |_, cx| {
            owner
                .update(cx, |view, cx| {
                    if view.busy {
                        view.status = "Info: wait for the current operation to finish".into();
                        cx.notify();
                        false
                    } else {
                        view.persist(cx)
                    }
                })
                .unwrap_or(true)
        });
        view.persist(cx);
        cx.spawn(async move |owner, cx| {
            loop {
                cx.background_executor()
                    .timer(std::time::Duration::from_millis(350))
                    .await;
                if owner
                    .update(cx, |view, cx| {
                        let progress = if view.busy {
                            view.selected_app().as_ref().and_then(update::read_progress)
                        } else {
                            None
                        };
                        if progress != view.progress {
                            view.progress = progress;
                            cx.notify();
                        }
                    })
                    .is_err()
                {
                    break;
                }
            }
        })
        .detach();
        view
    }
    fn select(&mut self, index: usize, cx: &mut Context<Self>) {
        if self.busy {
            return;
        }
        self.selected = Some(index);
        self.checked = None;
        self.confirm_uninstall = false;
        self.page = Page::Library;
        if let Some(app) = self.selected_app() {
            self.config.last_selected_path = Some(app.path.clone());
            let key = app
                .path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned();
            let repo = self
                .config
                .repository_drafts
                .get(&key)
                .cloned()
                .or(app.repository)
                .unwrap_or_default();
            self.source.update(cx, |field, cx| field.set_text(repo, cx));
            self.persist(cx);
        }
        cx.notify();
    }
    fn selected_app(&self) -> Option<AppInfo> {
        self.selected.and_then(|i| self.apps.get(i)).cloned()
    }
    fn job(
        &mut self,
        label: impl Into<String>,
        cx: &mut Context<Self>,
        task: impl FnOnce() -> anyhow::Result<JobResult> + Send + 'static,
    ) {
        if self.busy {
            return;
        }
        self.busy = true;
        self.error = false;
        self.status = label.into();
        let (tx, rx) = futures::channel::oneshot::channel();
        std::thread::spawn(move || {
            let _ = tx.send(task());
        });
        cx.spawn(async move |view, cx| {
            let result = rx.await.unwrap_or_else(|e| Err(e.into()));
            let _ = view.update(cx, |view, cx| {
                view.busy = false;
                view.progress = None;
                match result {
                    Ok(JobResult::Apps(apps, message)) => {
                        let old = view.selected_app().map(|a| a.path);
                        view.apps = apps;
                        let index = old
                            .and_then(|p| view.apps.iter().position(|a| a.path == p))
                            .or_else(|| (!view.apps.is_empty()).then_some(0));
                        view.selected = None;
                        let previous_page = view.page;
                        if let Some(i) = index {
                            view.select(i, cx);
                        }
                        if previous_page != Page::Library {
                            view.page = previous_page;
                        }
                        view.status = message;
                    }
                    Ok(JobResult::Check(check)) => {
                        view.status = check.status.clone();
                        view.checked = Some(check);
                    }
                    Ok(JobResult::Message(message)) => view.status = message,
                    Err(e) => {
                        view.status = tr(format!("操作失败：{e:#}"));
                        view.error = true;
                    }
                }
                cx.notify();
            });
        })
        .detach();
        cx.notify();
    }
    fn refresh(&mut self, cx: &mut Context<Self>) {
        let config = self.config.clone();
        self.job(tr("正在扫描应用…"), cx, move || {
            Ok(JobResult::Apps(
                manager::scan(&config)?,
                tr("应用列表已刷新"),
            ))
        });
    }
    fn import(&mut self, cx: &mut Context<Self>) {
        self.page = Page::Library;
        let path = PathBuf::from(self.import_path.read(cx).text());
        let config = self.config.clone();
        self.job(tr("正在导入应用…"), cx, move || {
            let app = manager::import_app(&config, &path)?;
            Ok(JobResult::Apps(
                manager::scan(&config)?,
                tr(format!("已导入 {}", app.name)),
            ))
        });
    }
    fn check_update(&mut self, cx: &mut Context<Self>) {
        if let Some(app) = self.selected_app() {
            self.job(tr("正在检查最新稳定版…"), cx, move || {
                Ok(JobResult::Check(update::check_update(&app)?))
            });
        }
    }
    fn install_update(&mut self, cx: &mut Context<Self>) {
        if let (Some(app), Some(asset)) = (
            self.selected_app(),
            self.checked.as_ref().and_then(|c| c.asset.clone()),
        ) {
            let config = self.config.clone();
            self.job(
                tr("正在下载并安装更新，请稍候…"),
                cx,
                move || {
                    let app = update::install_update(&config, &app, &asset)?;
                    Ok(JobResult::Apps(
                        manager::scan(&config)?,
                        tr(format!("{} 已更新，旧版本已保留用于回滚", app.name)),
                    ))
                },
            );
        }
    }
    fn uninstall(&mut self, cx: &mut Context<Self>) {
        if let Some(app) = self.selected_app() {
            let config = self.config.clone();
            self.confirm_uninstall = false;
            self.job(tr("正在卸载应用…"), cx, move || {
                manager::uninstall(&config, &app)?;
                Ok(JobResult::Apps(
                    manager::scan(&config)?,
                    tr(format!("{} 已移至回收站，个人数据已保留", app.name)),
                ))
            });
        }
    }
    fn save_directory(&mut self, cx: &mut Context<Self>) {
        if self.busy {
            return;
        }
        let value = self.directory.read(cx).text().trim().to_owned();
        let path = if let Some(rest) = value.strip_prefix("~/") {
            dirs::home_dir().unwrap_or_default().join(rest)
        } else {
            PathBuf::from(value)
        };
        let result = (|| -> anyhow::Result<()> {
            anyhow::ensure!(path.is_absolute(), tr("请选择绝对路径"));
            std::fs::create_dir_all(&path)?;
            let mut config = self.config.clone();
            config.applications_dir = path.canonicalize()?;
            config.directory_draft = None;
            config.save()?;
            self.config = config;
            Ok(())
        })();
        match result {
            Ok(()) => {
                self.page = Page::Library;
                self.refresh(cx);
            }
            Err(e) => {
                self.error = true;
                self.status = tr(format!("保存失败：{e:#}"));
                cx.notify();
            }
        }
    }
    fn save_repository(&mut self, cx: &mut Context<Self>) {
        if self.busy {
            return;
        }
        if let Some(app) = self.selected_app() {
            let value = self.source.read(cx).text().to_owned();
            match manager::set_repository(&mut self.config, &app, &value) {
                Ok(()) => {
                    if let Some(name) = app.path.file_name() {
                        self.config
                            .repository_drafts
                            .remove(&name.to_string_lossy().into_owned());
                    }
                    self.persist(cx);
                    self.refresh(cx);
                }
                Err(e) => {
                    self.error = true;
                    self.status = tr(format!("更新源保存失败：{e:#}"));
                    cx.notify();
                }
            }
        }
    }

    fn persist(&mut self, cx: &mut Context<Self>) -> bool {
        match self.config.save() {
            Ok(()) => true,
            Err(e) => {
                self.error = true;
                self.status = tr(format!("设置保存失败：{e:#}"));
                cx.notify();
                false
            }
        }
    }
    fn toggle_theme(&mut self, cx: &mut Context<Self>) {
        let mut next = self.config.clone();
        next.appearance = if cx.theme().is_dark() {
            Appearance::Light
        } else {
            Appearance::Dark
        };
        match next.save() {
            Ok(()) => {
                self.config = next;
                Theme::set_mode(
                    if self.config.appearance == Appearance::Dark {
                        Mode::Dark
                    } else {
                        Mode::Light
                    },
                    cx,
                );
                self.error = false;
                self.status = tr("主题已保存，重启后恢复");
            }
            Err(e) => {
                self.error = true;
                self.status = tr(format!("主题保存失败：{e:#}"));
            }
        }
        cx.notify();
    }
    fn set_window_style(&mut self, style: WindowStyle, cx: &mut Context<Self>) {
        if self.busy {
            return;
        }
        let mut next = self.config.clone();
        next.window_style = style;
        match next.save() {
            Ok(()) => {
                self.config = next;
                self.error = false;
                self.status = tr("窗口工具包已保存，正在重启…");
                match std::env::current_exe().and_then(|exe| Command::new(exe).spawn()) {
                    Ok(_) => cx.quit(),
                    Err(e) => {
                        self.error = true;
                        self.status = tr(format!("无法重启：{e}"));
                    }
                }
            }
            Err(e) => {
                self.error = true;
                self.status = tr(format!("窗口样式保存失败：{e:#}"));
            }
        }
        cx.notify();
    }

    fn set_language(&mut self, language: String, cx: &mut Context<Self>) {
        if self.busy {
            return;
        }
        let mut next = self.config.clone();
        next.language = language;
        match next.save() {
            Ok(()) => {
                self.config = next;
                match std::env::current_exe().and_then(|exe| Command::new(exe).spawn()) {
                    Ok(_) => cx.quit(),
                    Err(e) => {
                        self.error = true;
                        self.status = format!("Error: failed to restart application: {e}");
                    }
                }
            }
            Err(e) => {
                self.error = true;
                self.status = format!("Error: failed to save language: {e}");
            }
        }
        cx.notify();
    }
    fn import_dropped(&mut self, paths: Vec<PathBuf>, cx: &mut Context<Self>) {
        if paths.is_empty() {
            return;
        }
        let config = self.config.clone();
        self.page = Page::Library;
        self.job(tr("正在导入拖放的 AppImage…"), cx, move || {
            let mut count = 0;
            for path in paths {
                manager::import_app(&config, &path)?;
                count += 1;
            }
            Ok(JobResult::Apps(
                manager::scan(&config)?,
                tr(format!("已导入 {count} 个应用")),
            ))
        });
    }
    fn open_directory(&mut self, cx: &mut Context<Self>) {
        let path = self.config.applications_dir.clone();
        self.job(tr("正在打开应用目录…"), cx, move || {
            let result = Command::new("xdg-open").arg(path).status()?;
            anyhow::ensure!(result.success(), tr("文件管理器无法打开目录"));
            Ok(JobResult::Message(tr("应用目录已打开")))
        });
    }
    fn choose_import(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.busy {
            return;
        }
        self.page = Page::Settings;
        self.status = tr("选择 AppImage 文件，然后点击导入");
        self.import_path.update(cx, |field, cx| {
            field.focus().focus(window, cx);
            cx.notify();
        });
        cx.notify();
    }
    fn topbar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors.clone();
        div()
            .flex()
            .items_center()
            .justify_between()
            .h(px(62.))
            .px_5()
            .border_b_1()
            .border_color(colors.border)
            .bg(colors.surface)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(
                        Image::new(
                            "manager-brand",
                            dirs::cache_dir()
                                .unwrap_or_default()
                                .join("appimage-mannger/branding/appimage-mannger.png"),
                        )
                        .size(px(36.))
                        .fit(gpui::ObjectFit::Contain),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .child(tr("AppimageMannger 管理器")),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(colors.fg_muted)
                                    .child(tr("你的 AppImage 应用库")),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        Button::new("refresh", tr("刷新"))
                            .icon(IconName::RefreshCw)
                            .variant(ButtonVariant::Ghost)
                            .disabled(self.busy)
                            .on_click(cx.listener(|this, _, _, cx| this.refresh(cx))),
                    )
                    .child(
                        Button::new("import", tr("导入应用"))
                            .icon(IconName::Plus)
                            .primary()
                            .disabled(self.busy)
                            .on_click(
                                cx.listener(|this, _, window, cx| this.choose_import(window, cx)),
                            ),
                    ),
            )
    }
    fn sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors.clone();
        let query = self.search.read(cx).text().to_lowercase();
        let rows = self
            .apps
            .iter()
            .enumerate()
            .filter(|(_, a)| a.name.to_lowercase().contains(&query))
            .map(|(i, app)| {
                let selected = self.selected == Some(i) && self.page == Page::Library;
                ListItem::new(("app", i), app.name.clone())
                    .description(format!("{} · {}", app.version, human_size(app.size_bytes)))
                    .leading(app_icon(app, ("list-icon", i), 38.0, cx))
                    .trailing(
                        Icon::new(IconName::ChevronRight)
                            .size(IconSize::Sm)
                            .color(colors.fg_muted),
                    )
                    .selected(selected)
                    .disabled(self.busy)
                    .place(i + 1, self.apps.len())
                    .on_click(cx.listener(move |this, _, _, cx| this.select(i, cx)))
            })
            .collect::<Vec<_>>();
        div()
            .w_full()
            .flex_shrink_0()
            .h_full()
            .bg(colors.surface)
            .border_r_1()
            .border_color(colors.border)
            .flex()
            .flex_col()
            .child(
                div()
                    .p_4()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .text_xs()
                            .font_weight(gpui::FontWeight::MEDIUM)
                            .text_color(colors.fg_muted)
                            .child(tr("已安装的应用"))
                            .child(format!("{}", self.apps.len())),
                    )
                    .child(
                        Input::new(&self.search)
                            .prefix(Icon::new(IconName::Search).size(IconSize::Sm))
                            .clearable(),
                    ),
            )
            .child(
                div()
                    .id("application-list")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .px_3()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .children(rows),
            )
            .child(
                div().border_t_1().border_color(colors.border).p_3().child(
                    Button::new("settings", tr("设置与导入"))
                        .icon(IconName::Settings)
                        .variant(ButtonVariant::Ghost)
                        .full_width()
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.page = Page::Settings;
                            cx.notify();
                        })),
                ),
            )
    }
    fn detail(&self, cx: &mut Context<Self>) -> gpui::AnyElement {
        let colors = cx.theme().colors.clone();
        let Some(app) = self.selected_app() else {
            return div()
                .size_full()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap_4()
                .child(
                    Icon::new(IconName::Package)
                        .size(IconSize::Lg)
                        .color(colors.accent),
                )
                .child(div().text_xl().child(tr("让应用各就其位")))
                .child(
                    div()
                        .text_sm()
                        .text_color(colors.fg_muted)
                        .child(tr("导入 AppImage，或在设置中选择已有的应用目录。")),
                )
                .child(
                    Button::new("empty-import", tr("导入第一个应用"))
                        .primary()
                        .on_click(
                            cx.listener(|this, _, window, cx| this.choose_import(window, cx)),
                        ),
                )
                .into_any_element();
        };
        let header = div()
            .flex()
            .items_center()
            .gap_5()
            .child(app_icon(&app, "detail-app-icon", 76.0, cx))
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_xs()
                            .text_color(colors.accent)
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .child(tr("APPIMAGE 应用")),
                    )
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .child(app.name.clone()),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(colors.fg_muted)
                            .child(tr(format!(
                                "版本 {}   ·   {}",
                                app.version,
                                human_size(app.size_bytes)
                            ))),
                    ),
            )
            .child(
                Button::new("launch", tr("启动应用"))
                    .icon(IconName::Play)
                    .primary()
                    .disabled(self.busy)
                    .on_click(cx.listener(|this, _, _, cx| {
                        if let Some(app) = this.selected_app() {
                            this.job(tr("正在启动应用…"), cx, move || {
                                manager::launch(&app)?;
                                Ok(JobResult::Message(tr("应用已启动")))
                            });
                        }
                    })),
            );
        let mut body = div()
            .flex()
            .flex_col()
            .gap_6()
            .child(header)
            .child(
                card(tr("应用信息"), colors.border, colors.surface)
                    .child(info(
                        tr("描述"),
                        if app.description.is_empty() {
                            tr("此应用未提供描述")
                        } else {
                            app.description.clone()
                        },
                        &colors,
                    ))
                    .child(info(
                        tr("安装位置"),
                        app.path.to_string_lossy().to_string(),
                        &colors,
                    ))
                    .child(info(tr("文件大小"), human_size(app.size_bytes), &colors))
                    .child(info(tr("运行架构"), app.architecture.clone(), &colors))
                    .child(
                        div()
                            .pt_2()
                            .flex()
                            .gap_2()
                            .child(
                                Button::new("repair-menu", tr("修复菜单入口"))
                                    .icon(IconName::Wrench)
                                    .variant(ButtonVariant::Ghost)
                                    .disabled(self.busy)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        if let Some(app) = this.selected_app() {
                                            let config = this.config.clone();
                                            this.job(
                                                tr("正在创建菜单入口…"),
                                                cx,
                                                move || {
                                                    manager::integrate_app(&config, &app)?;
                                                    Ok(JobResult::Message(tr("菜单入口已创建")))
                                                },
                                            );
                                        }
                                    })),
                            )
                            .child(
                                Button::new("show-folder", tr("打开所在目录"))
                                    .icon(IconName::FolderOpen)
                                    .variant(ButtonVariant::Ghost)
                                    .disabled(self.busy)
                                    .on_click(
                                        cx.listener(|this, _, _, cx| this.open_directory(cx)),
                                    ),
                            ),
                    ),
            )
            .child(
                card(tr("版本与更新"), colors.border, colors.surface)
                    .child(div().text_sm().text_color(colors.fg_muted).child(tr(
                        "检查 GitHub 最新稳定版；发现适配当前架构的 AppImage 后可安装。",
                    )))
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .items_end()
                            .child(div().flex_1().child(Input::new(&self.source)))
                            .child(
                                Button::new("save-source", tr("保存更新源"))
                                    .disabled(self.busy)
                                    .on_click(
                                        cx.listener(|this, _, _, cx| this.save_repository(cx)),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(
                                Button::new("check-update", tr("检查更新"))
                                    .icon(IconName::RefreshCw)
                                    .disabled(self.busy)
                                    .on_click(cx.listener(|this, _, _, cx| this.check_update(cx))),
                            )
                            .child(
                                Button::new("install-update", tr("安装更新"))
                                    .icon(IconName::Download)
                                    .primary()
                                    .disabled(
                                        self.busy
                                            || self
                                                .checked
                                                .as_ref()
                                                .and_then(|c| c.asset.as_ref())
                                                .is_none(),
                                    )
                                    .on_click(
                                        cx.listener(|this, _, _, cx| this.install_update(cx)),
                                    ),
                            ),
                    )
                    .children(self.checked.as_ref().map(|check| {
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .p_3()
                            .rounded_lg()
                            .bg(colors.sunken)
                            .child(check.status.clone())
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(colors.fg_muted)
                                    .child(tr(format!(
                                        "当前 {}  →  最新 {}",
                                        check.current_version, check.latest_version
                                    ))),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(colors.fg_muted)
                                    .child(check.notes.chars().take(1200).collect::<String>()),
                            )
                    })),
            )
            .child(
                card(tr("应用管理"), colors.border, colors.surface).child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap_4()
                        .child(
                            div().flex().flex_col().gap_1().child(tr("卸载应用")).child(
                                div()
                                    .text_sm()
                                    .text_color(colors.fg_muted)
                                    .child(tr("移至回收站并移除菜单入口，保留个人配置。")),
                            ),
                        )
                        .child(
                            Button::new("uninstall", tr("卸载…"))
                                .icon(IconName::Trash2)
                                .variant(ButtonVariant::Danger)
                                .disabled(self.busy)
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.confirm_uninstall = true;
                                    cx.notify();
                                })),
                        ),
                ),
            );
        if self.confirm_uninstall {
            body =
                body.child(
                    div()
                        .p_4()
                        .rounded_lg()
                        .border_1()
                        .border_color(colors.danger)
                        .flex()
                        .flex_col()
                        .gap_3()
                        .child(tr(format!("确认将 {} 移至回收站？", app.name)))
                        .child(
                            div()
                                .flex()
                                .gap_2()
                                .child(
                                    Button::new("confirm-remove", tr("确认卸载"))
                                        .variant(ButtonVariant::Danger)
                                        .on_click(cx.listener(|this, _, _, cx| this.uninstall(cx))),
                                )
                                .child(Button::new("cancel-remove", tr("取消")).on_click(
                                    cx.listener(|this, _, _, cx| {
                                        this.confirm_uninstall = false;
                                        cx.notify();
                                    }),
                                )),
                        ),
                );
        }
        body.into_any_element()
    }
    fn settings(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors.clone();
        let this = cx.entity();
        let choices = std::iter::once(Choice::new("system", tr("跟随系统")))
            .chain(
                crate::i18n::locale_list()
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|l| {
                        Choice::new(
                            l["code"].as_str().unwrap().to_owned(),
                            l["name"].as_str().unwrap().to_owned(),
                        )
                    }),
            )
            .collect::<Vec<_>>();
        div()
            .flex()
            .flex_col()
            .gap_6()
            .child(
                div()
                    .text_2xl()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .child(tr("设置")),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(colors.fg_muted)
                    .child(tr("为你的 AppImage 选择一个固定的家。")),
            )

            .child(card(tr("语言"),colors.border,colors.surface).child(Select::new("language-selector",choices).selected(self.config.language.clone()).label(tr("语言")).on_change(cx.listener(|this,value:&SharedString,_,cx|this.set_language(value.to_string(),cx)))).child(tr("更改语言后保存并重启应用。")))
            .child(card(tr("窗口效果"),colors.border,colors.surface)
                .child(SegmentedControl::new("window-style",match self.config.window_style{WindowStyle::Gpui=>"gpui",WindowStyle::Qt=>"qt",WindowStyle::Gnome=>"gnome"})
                    .segment("gpui","GPUI",None).segment("qt","Qt",None).segment("gnome","GNOME",None)
                    .on_change(cx.listener(|this,value:&SharedString,_,cx|{let style=match value.as_ref(){"qt"=>WindowStyle::Qt,"gnome"=>WindowStyle::Gnome,_=>WindowStyle::Gpui};this.set_window_style(style,cx);})))
                .child(div().text_sm().text_color(colors.fg_muted).child(tr("GPUI 使用 Ely，Qt 使用 Qt 6，GNOME 使用 GTK 4/libadwaita。选择后保存并重启，后续启动保持此工具包。"))))
            .child(
                card(tr("应用目录"), colors.border, colors.surface)
                    .child(PathInput::new("application-directory", &self.directory).directories())
                    .child(
                        div().text_sm().text_color(colors.fg_muted).child(
                            tr("默认 ~/Applications。输入会保存为草稿，点击保存并扫描后应用；已有应用不会被移动。"),
                        ),
                    )
                    .child(
                        div().child(
                            Button::new("save-directory", tr("保存并扫描"))
                                .primary()
                                .disabled(self.busy)
                                .on_click(cx.listener(|this, _, _, cx| this.save_directory(cx))),
                        ),
                    ),
            )
            .child(
                card(tr("导入本地 AppImage"), colors.border, colors.surface)
                    .child(PathInput::new("import-file", &self.import_path).on_choose(
                        move |path, _, cx| {
                            this.update(cx, |view, cx| {
                                view.import_path.update(cx, |f, cx| {
                                    f.set_text(path.to_string_lossy().to_string(), cx)
                                });
                                cx.notify();
                            });
                        },
                    ))
                    .child(
                        div()
                            .text_sm()
                            .text_color(colors.fg_muted)
                            .child(tr("复制到应用目录并添加菜单入口。源文件会保留。")),
                    )
                    .child(
                        div().child(
                            Button::new("do-import", tr("导入应用"))
                                .icon(IconName::Plus)
                                .primary()
                                .disabled(self.busy)
                                .on_click(cx.listener(|this, _, _, cx| this.import(cx))),
                        ),
                    ),
            )
            .child(
                card(tr("外观"), colors.border, colors.surface).child(
                    div().child(
                        Button::new("toggle-theme", tr("切换明暗主题"))
                            .icon(IconName::Moon)
                            .on_click(cx.listener(|this,_,_,cx|this.toggle_theme(cx))),
                    ),
                ),
            )
    }
    fn about(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors.clone();
        div()
            .flex()
            .flex_col()
            .gap_5()
            .child(
                Icon::new(IconName::Package)
                    .size(IconSize::Lg)
                    .color(colors.accent),
            )
            .child(
                div()
                    .text_2xl()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .child(tr("AppimageMannger 管理器")),
            )
            .child(tr(format!(
                "版本 {} · AGPL-3.0-only",
                env!("CARGO_PKG_VERSION")
            )))
            .child(
                div()
                    .text_color(colors.fg_muted)
                    .child(tr("用 Ely + GPUI 构建的原生 Linux AppImage 管理器。")),
            )
            .child(
                card(tr("项目信息"), colors.border, colors.surface)
                    .child(tr("作者：xingwangzhe"))
                    .child(tr("邮箱：xingwangzhe@outlook.com"))
                    .child(tr("架构：x86_64 / aarch64"))
                    .child(tr(
                        "更新通过上游 GitHub 发布获取；没有配置更新源时会明确提示。",
                    )),
            )
            .child(
                div().child(
                    Button::new("repository", tr("项目主页"))
                        .icon(IconName::ExternalLink)
                        .on_click(|_, _, cx| {
                            cx.open_url("https://github.com/xingwangzhe/AppimageMannger")
                        }),
                ),
            )
    }
}
fn human_size(bytes: u64) -> String {
    if bytes >= 1024 * 1024 {
        format!("{:.1} MiB", bytes as f64 / 1048576.)
    } else {
        format!("{:.1} KiB", bytes as f64 / 1024.)
    }
}
fn card(title: String, _border: gpui::Hsla, _bg: gpui::Hsla) -> Card {
    Card::new().header(CardHeader::new(title))
}
fn app_icon(
    app: &AppInfo,
    id: impl Into<gpui::ElementId>,
    edge: f32,
    cx: &App,
) -> gpui::AnyElement {
    let id = id.into();
    if let Some(path) = &app.icon_path {
        Image::new(id, path.clone())
            .fit(gpui::ObjectFit::Contain)
            .size(px(edge))
            .rounded_lg()
            .into_any_element()
    } else {
        div()
            .size(px(edge))
            .rounded_lg()
            .bg(cx.theme().colors.sunken)
            .flex()
            .items_center()
            .justify_center()
            .child(Icon::new(IconName::Package).color(cx.theme().colors.accent))
            .into_any_element()
    }
}

fn info(label: String, value: String, colors: &ely_core::theme::Palette) -> gpui::Div {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(div().text_xs().text_color(colors.fg_muted).child(label))
        .child(div().text_sm().child(value))
}
impl Render for ManagerView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors.clone();
        let page = match self.page {
            Page::Library => self.detail(cx),
            Page::Settings => self.settings(cx).into_any_element(),
            Page::About => self.about(cx).into_any_element(),
        };
        let owner = cx.weak_entity();
        let rtl = crate::i18n::is_rtl();
        let sizes = if rtl {
            [
                1.0 - self.config.sidebar_fraction,
                self.config.sidebar_fraction,
            ]
        } else {
            [
                self.config.sidebar_fraction,
                1.0 - self.config.sidebar_fraction,
            ]
        };
        let split = SplitPane::new("library-split", gpui::Axis::Horizontal, px(200.))
            .sizes(&sizes)
            .on_resize(move |sizes, _, cx| {
                let _ = owner.update(cx, |view, cx| {
                    view.config.sidebar_fraction = sizes[usize::from(rtl)].clamp(0.15, 0.7);
                    view.persist(cx);
                });
            });
        let detail = div()
            .id("detail-scroll")
            .size_full()
            .overflow_y_scroll()
            .p_8()
            .child(page);
        let split = if rtl {
            split.pane(detail).pane(self.sidebar(cx))
        } else {
            split.pane(self.sidebar(cx)).pane(detail)
        };
        div()
            .id("app-window")
            .size_full()
            .on_drop(cx.listener(|this, paths: &gpui::ExternalPaths, _, cx| {
                this.import_dropped(paths.paths().to_vec(), cx)
            }))
            .on_action(cx.listener(|this, _: &Refresh, _, cx| this.refresh(cx)))
            .on_action(cx.listener(|this, _: &Import, window, cx| this.choose_import(window, cx)))
            .on_action(cx.listener(|this, _: &Settings, _, cx| {
                this.page = Page::Settings;
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &About, _, cx| {
                this.page = Page::About;
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ToggleTheme, _, cx| this.toggle_theme(cx)))
            .on_action(cx.listener(|this, _: &Quit, _, cx| {
                if !this.busy && this.persist(cx) {
                    cx.quit();
                } else {
                    this.status = "Info: wait for the current operation to finish".into();
                    cx.notify();
                }
            }))
            .child(
                FocusScope::new(&self.focus)
                    .root()
                    .size_full()
                    .flex()
                    .flex_col()
                    .bg(colors.bg)
                    .text_color(colors.fg)
                    .text_sm()
                    .child(crate::chrome::chrome(self.config.window_style, window, cx))
                    .child(
                        div()
                            .bg(colors.surface)
                            .border_b_1()
                            .border_color(colors.border)
                            .child(MenuBar::new("menu-bar", cx.get_menus().unwrap_or_default())),
                    )
                    .child(self.topbar(cx))
                    .child(div().flex().flex_1().min_h_0().child(split))
                    .child(
                        div()
                            .h(px(36.))
                            .flex_shrink_0()
                            .px_4()
                            .flex()
                            .items_center()
                            .justify_between()
                            .border_t_1()
                            .border_color(colors.border)
                            .bg(colors.surface)
                            .when_some(self.progress.as_ref(), |row, progress| {
                                let fraction = progress.total.filter(|t| *t > 0).map(|total| {
                                    (progress.downloaded as f32 / total as f32).clamp(0., 1.)
                                });
                                let text = if let (Some(total), Some(fraction)) =
                                    (progress.total, fraction)
                                {
                                    format!(
                                        "{:.0}% · {} / {}",
                                        fraction * 100.,
                                        human_size(progress.downloaded),
                                        human_size(total)
                                    )
                                } else {
                                    human_size(progress.downloaded)
                                };
                                row.child(
                                    div()
                                        .w(px(230.))
                                        .flex()
                                        .flex_col()
                                        .gap_1()
                                        .child(div().text_xs().child(text))
                                        .child(if let Some(fraction) = fraction {
                                            ProgressBar::new("download-progress", fraction)
                                        } else {
                                            ProgressBar::indeterminate("download-progress")
                                        }),
                                )
                            })
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(if self.error {
                                        colors.danger
                                    } else {
                                        colors.fg_muted
                                    })
                                    .child(self.status.clone()),
                            )
                            .child(div().text_xs().text_color(colors.fg_muted).child(
                                if self.busy {
                                    tr("处理中…").to_owned()
                                } else {
                                    tr(format!(
                                        "{} 个应用 · {}",
                                        self.apps.len(),
                                        std::env::consts::ARCH
                                    ))
                                },
                            )),
                    ),
            )
    }
}
pub fn run() {
    gpui_platform::application()
        .with_assets(Assets)
        .run(|cx: &mut App| {
            ely_core::init(cx).expect("Ely initialization failed");
            let preferences = Config::load().unwrap_or_default();
            crate::i18n::set_locale(&preferences.language);
            cx.set_app_identity("appimage-mannger", &tr("AppimageMannger 管理器"));
            cx.set_global(crate::i18n::ely_i18n(&preferences.language));
            Theme::set_mode_now(
                if preferences.appearance == Appearance::Dark {
                    Mode::Dark
                } else {
                    Mode::Light
                },
                cx,
            );
            cx.bind_keys([
                KeyBinding::new("ctrl-r", Refresh, None),
                KeyBinding::new("ctrl-o", Import, None),
                KeyBinding::new("ctrl-,", Settings, None),
                KeyBinding::new("ctrl-q", Quit, None),
            ]);
            cx.set_menus(vec![
                Menu {
                    disabled: false,
                    name: tr("文件").into(),
                    items: vec![
                        MenuItem::action(tr("导入 AppImage"), Import),
                        MenuItem::action(tr("刷新应用列表"), Refresh),
                        MenuItem::separator(),
                        MenuItem::action(tr("退出"), Quit),
                    ],
                },
                Menu {
                    disabled: false,
                    name: tr("工具").into(),
                    items: vec![
                        MenuItem::action(tr("设置"), Settings),
                        MenuItem::action(tr("切换明暗主题"), ToggleTheme),
                    ],
                },
                Menu {
                    disabled: false,
                    name: tr("帮助").into(),
                    items: vec![MenuItem::action(tr("关于 AppimageMannger"), About)],
                },
            ]);
            let bounds = Bounds::centered(
                None,
                size(px(preferences.window_width), px(preferences.window_height)),
                cx,
            );
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(if preferences.maximized {
                        WindowBounds::Maximized(bounds)
                    } else {
                        WindowBounds::Windowed(bounds)
                    }),
                    titlebar: Some(TitlebarOptions {
                        title: Some(SharedString::from(tr("AppimageMannger 管理器"))),
                        ..Default::default()
                    }),
                    window_min_size: Some(size(px(880.), px(580.))),
                    app_id: Some("appimage-mannger".into()),
                    icon: Some(std::sync::Arc::new(
                        image::load_from_memory(include_bytes!(
                            "../packaging/appimage-mannger.png"
                        ))
                        .expect("Bundled application icon")
                        .to_rgba8(),
                    )),
                    ..Default::default()
                },
                |window, cx| cx.new(|cx| ManagerView::new(window, cx)),
            )
            .expect("Failed to open window");
            cx.activate(true);
        });
}
