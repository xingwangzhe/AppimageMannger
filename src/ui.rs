use crate::{
    manager::{self, AppInfo, Config},
    update::{self, UpdateCheck},
};
use ely_core::{
    Assets,
    buttons::{Button, ButtonVariant},
    forms::{Input, InputEvent, PathInput, TextInput},
    menus::MenuBar,
    primitives::{FocusScope, Icon, IconName},
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
}

impl ManagerView {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let loaded = Config::load();
        let (config, mut status, mut error) = match loaded {
            Ok(c) => (c, "准备就绪".to_owned(), false),
            Err(e) => (Config::default(), format!("配置读取失败：{e:#}"), true),
        };
        let focus = cx.focus_handle();
        window.focus(&focus, cx);
        let search = cx.new(|cx| {
            TextInput::new(window, cx)
                .label("搜索应用")
                .placeholder("搜索应用名称…")
        });
        cx.subscribe(&search, |_, _, _: &InputEvent, cx| cx.notify())
            .detach();
        let directory = cx.new(|cx| {
            let mut input = TextInput::new(window, cx).label("应用安装目录");
            input.set_text(config.applications_dir.to_string_lossy().to_string(), cx);
            input
        });
        let source = cx.new(|cx| {
            TextInput::new(window, cx)
                .label("GitHub 更新仓库")
                .placeholder("owner/repository")
        });
        let import_path = cx.new(|cx| {
            TextInput::new(window, cx)
                .label("导入 AppImage")
                .placeholder("选择本地 .AppImage 文件")
        });
        let apps = match manager::scan(&config) {
            Ok(apps) => apps,
            Err(e) => {
                status = format!("扫描失败：{e:#}");
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
        };
        if !view.apps.is_empty() {
            view.select(0, cx);
        }
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
        let repo = self
            .apps
            .get(index)
            .and_then(|a| a.repository.clone())
            .unwrap_or_default();
        self.source.update(cx, |field, cx| field.set_text(repo, cx));
        cx.notify();
    }
    fn selected_app(&self) -> Option<AppInfo> {
        self.selected.and_then(|i| self.apps.get(i)).cloned()
    }
    fn job(
        &mut self,
        label: &str,
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
                match result {
                    Ok(JobResult::Apps(apps, message)) => {
                        let old = view.selected_app().map(|a| a.path);
                        view.apps = apps;
                        let index = old
                            .and_then(|p| view.apps.iter().position(|a| a.path == p))
                            .or_else(|| (!view.apps.is_empty()).then_some(0));
                        view.selected = None;
                        if let Some(i) = index {
                            view.select(i, cx);
                        }
                        view.status = message;
                    }
                    Ok(JobResult::Check(check)) => {
                        view.status = format!("{}", check.status);
                        view.checked = Some(check);
                    }
                    Ok(JobResult::Message(message)) => view.status = message,
                    Err(e) => {
                        view.status = format!("操作失败：{e:#}");
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
        self.job("正在扫描应用…", cx, move || {
            Ok(JobResult::Apps(
                manager::scan(&config)?,
                "应用列表已刷新".into(),
            ))
        });
    }
    fn import(&mut self, cx: &mut Context<Self>) {
        let path = PathBuf::from(self.import_path.read(cx).text());
        let config = self.config.clone();
        self.job("正在导入应用…", cx, move || {
            let app = manager::import_app(&config, &path)?;
            Ok(JobResult::Apps(
                manager::scan(&config)?,
                format!("已导入 {}", app.name),
            ))
        });
    }
    fn check_update(&mut self, cx: &mut Context<Self>) {
        if let Some(app) = self.selected_app() {
            self.job("正在检查最新稳定版…", cx, move || {
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
                "正在下载并安装更新，请稍候…",
                cx,
                move || {
                    let app = update::install_update(&config, &app, &asset)?;
                    Ok(JobResult::Apps(
                        manager::scan(&config)?,
                        format!("{} 已更新，旧版本已保留用于回滚", app.name),
                    ))
                },
            );
        }
    }
    fn uninstall(&mut self, cx: &mut Context<Self>) {
        if let Some(app) = self.selected_app() {
            let config = self.config.clone();
            self.confirm_uninstall = false;
            self.job("正在卸载应用…", cx, move || {
                manager::uninstall(&config, &app)?;
                Ok(JobResult::Apps(
                    manager::scan(&config)?,
                    format!("{} 已移至回收站，个人数据已保留", app.name),
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
            anyhow::ensure!(path.is_absolute(), "请选择绝对路径");
            std::fs::create_dir_all(&path)?;
            let mut config = self.config.clone();
            config.applications_dir = path.canonicalize()?;
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
                self.status = format!("保存失败：{e:#}");
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
                Ok(()) => self.refresh(cx),
                Err(e) => {
                    self.error = true;
                    self.status = format!("更新源保存失败：{e:#}");
                    cx.notify();
                }
            }
        }
    }
    fn open_directory(&mut self, cx: &mut Context<Self>) {
        let path = self.config.applications_dir.clone();
        self.job("正在打开应用目录…", cx, move || {
            let result = Command::new("xdg-open").arg(path).status()?;
            anyhow::ensure!(result.success(), "文件管理器无法打开目录");
            Ok(JobResult::Message("应用目录已打开".into()))
        });
    }
    fn choose_import(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.busy {
            return;
        }
        self.page = Page::Settings;
        self.status = "选择 AppImage 文件，然后点击导入".into();
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
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .size(px(32.))
                            .rounded_lg()
                            .bg(colors.accent)
                            .text_color(colors.on_accent)
                            .child(Icon::new(IconName::Package).size(IconSize::Md)),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(gpui::FontWeight::SEMIBOLD)
                                    .child("AppimageMannger"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(colors.fg_muted)
                                    .child("你的 AppImage 应用库"),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        Button::new("refresh", "刷新")
                            .icon(IconName::RefreshCw)
                            .variant(ButtonVariant::Ghost)
                            .disabled(self.busy)
                            .on_click(cx.listener(|this, _, _, cx| this.refresh(cx))),
                    )
                    .child(
                        Button::new("import", "导入应用")
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
                div()
                    .id(("app", i))
                    .flex()
                    .items_center()
                    .gap_3()
                    .p_3()
                    .rounded_lg()
                    .cursor_pointer()
                    .bg(if selected {
                        colors.selection
                    } else {
                        colors.surface
                    })
                    .border_1()
                    .border_color(if selected {
                        colors.accent
                    } else {
                        colors.surface
                    })
                    .hover(|s| s.bg(colors.hover))
                    .on_click(cx.listener(move |this, _, _, cx| this.select(i, cx)))
                    .child(
                        div()
                            .size(px(38.))
                            .rounded_lg()
                            .bg(colors.sunken)
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(Icon::new(IconName::Package).color(colors.accent)),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .flex_1()
                            .min_w_0()
                            .child(
                                div()
                                    .text_sm()
                                    .text_ellipsis()
                                    .font_weight(gpui::FontWeight::MEDIUM)
                                    .child(app.name.clone()),
                            )
                            .child(div().text_xs().text_color(colors.fg_muted).child(format!(
                                "{}  ·  {}",
                                app.version,
                                human_size(app.size_bytes)
                            ))),
                    )
                    .child(
                        Icon::new(IconName::ChevronRight)
                            .size(IconSize::Sm)
                            .color(colors.fg_muted),
                    )
            })
            .collect::<Vec<_>>();
        div()
            .w(px(294.))
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
                            .child("已安装的应用")
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
                    Button::new("settings", "设置与导入")
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
                .child(div().text_xl().child("让应用各就其位"))
                .child(
                    div()
                        .text_sm()
                        .text_color(colors.fg_muted)
                        .child("导入 AppImage，或在设置中选择已有的应用目录。"),
                )
                .child(
                    Button::new("empty-import", "导入第一个应用")
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
            .child(
                div()
                    .size(px(76.))
                    .rounded_2xl()
                    .bg(colors.selection)
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        Icon::new(IconName::Package)
                            .size(IconSize::Lg)
                            .color(colors.accent),
                    ),
            )
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
                            .child("APPIMAGE 应用"),
                    )
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .child(app.name.clone()),
                    )
                    .child(div().text_sm().text_color(colors.fg_muted).child(format!(
                        "版本 {}   ·   {}",
                        app.version,
                        human_size(app.size_bytes)
                    ))),
            )
            .child(
                Button::new("launch", "启动应用")
                    .icon(IconName::Play)
                    .primary()
                    .disabled(self.busy)
                    .on_click(cx.listener(|this, _, _, cx| {
                        if let Some(app) = this.selected_app() {
                            this.job("正在启动应用…", cx, move || {
                                manager::launch(&app)?;
                                Ok(JobResult::Message("应用已启动".into()))
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
                card("应用信息", colors.border, colors.surface)
                    .child(info(
                        "描述",
                        if app.description.is_empty() {
                            "此应用未提供描述".into()
                        } else {
                            app.description.clone()
                        },
                        &colors,
                    ))
                    .child(info(
                        "安装位置",
                        app.path.to_string_lossy().to_string(),
                        &colors,
                    ))
                    .child(info("文件大小", human_size(app.size_bytes), &colors))
                    .child(info("运行架构", app.architecture.clone(), &colors))
                    .child(
                        div()
                            .pt_2()
                            .flex()
                            .gap_2()
                            .child(
                                Button::new("repair-menu", "修复菜单入口")
                                    .icon(IconName::Wrench)
                                    .variant(ButtonVariant::Ghost)
                                    .disabled(self.busy)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        if let Some(app) = this.selected_app() {
                                            let config = this.config.clone();
                                            this.job(
                                                "正在创建菜单入口…",
                                                cx,
                                                move || {
                                                    manager::integrate_app(&config, &app)?;
                                                    Ok(JobResult::Message("菜单入口已创建".into()))
                                                },
                                            );
                                        }
                                    })),
                            )
                            .child(
                                Button::new("show-folder", "打开所在目录")
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
                card("版本与更新", colors.border, colors.surface)
                    .child(
                        div().text_sm().text_color(colors.fg_muted).child(
                            "检查 GitHub 最新稳定版；发现适配当前架构的 AppImage 后可安装。",
                        ),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .items_end()
                            .child(div().flex_1().child(Input::new(&self.source)))
                            .child(
                                Button::new("save-source", "保存更新源")
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
                                Button::new("check-update", "检查更新")
                                    .icon(IconName::RefreshCw)
                                    .disabled(self.busy)
                                    .on_click(cx.listener(|this, _, _, cx| this.check_update(cx))),
                            )
                            .child(
                                Button::new("install-update", "安装更新")
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
                            .child(format!("{}", check.status))
                            .child(div().text_sm().text_color(colors.fg_muted).child(format!(
                                "当前 {}  →  最新 {}",
                                check.current_version, check.latest_version
                            )))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(colors.fg_muted)
                                    .child(check.notes.chars().take(1200).collect::<String>()),
                            )
                    })),
            )
            .child(
                card("应用管理", colors.border, colors.surface).child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap_4()
                        .child(
                            div().flex().flex_col().gap_1().child("卸载应用").child(
                                div()
                                    .text_sm()
                                    .text_color(colors.fg_muted)
                                    .child("移至回收站并移除菜单入口，保留个人配置。"),
                            ),
                        )
                        .child(
                            Button::new("uninstall", "卸载…")
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
            body = body.child(
                div()
                    .p_4()
                    .rounded_lg()
                    .border_1()
                    .border_color(colors.danger)
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(format!("确认将 {} 移至回收站？", app.name))
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                Button::new("confirm-remove", "确认卸载")
                                    .variant(ButtonVariant::Danger)
                                    .on_click(cx.listener(|this, _, _, cx| this.uninstall(cx))),
                            )
                            .child(Button::new("cancel-remove", "取消").on_click(cx.listener(
                                |this, _, _, cx| {
                                    this.confirm_uninstall = false;
                                    cx.notify();
                                },
                            ))),
                    ),
            );
        }
        body.into_any_element()
    }
    fn settings(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors.clone();
        let this = cx.entity();
        div()
            .flex()
            .flex_col()
            .gap_6()
            .child(
                div()
                    .text_2xl()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .child("设置"),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(colors.fg_muted)
                    .child("为你的 AppImage 选择一个固定的家。"),
            )
            .child(
                card("应用目录", colors.border, colors.surface)
                    .child(PathInput::new("application-directory", &self.directory).directories())
                    .child(
                        div().text_sm().text_color(colors.fg_muted).child(
                            "默认 ~/Applications。更改目录后扫描新位置，已有应用不会被移动。",
                        ),
                    )
                    .child(
                        div().child(
                            Button::new("save-directory", "保存并扫描")
                                .primary()
                                .disabled(self.busy)
                                .on_click(cx.listener(|this, _, _, cx| this.save_directory(cx))),
                        ),
                    ),
            )
            .child(
                card("导入本地 AppImage", colors.border, colors.surface)
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
                            .child("复制到应用目录并添加菜单入口。源文件会保留。"),
                    )
                    .child(
                        div().child(
                            Button::new("do-import", "导入应用")
                                .icon(IconName::Plus)
                                .primary()
                                .disabled(self.busy)
                                .on_click(cx.listener(|this, _, _, cx| this.import(cx))),
                        ),
                    ),
            )
            .child(
                card("外观", colors.border, colors.surface).child(
                    div().child(
                        Button::new("toggle-theme", "切换明暗主题")
                            .icon(IconName::Moon)
                            .on_click(|_, _, cx| {
                                let mode = if cx.theme().is_dark() {
                                    Mode::Light
                                } else {
                                    Mode::Dark
                                };
                                Theme::set_mode(mode, cx);
                            }),
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
                    .child("AppimageMannger"),
            )
            .child(format!("版本 {} · MIT License", env!("CARGO_PKG_VERSION")))
            .child(
                div()
                    .text_color(colors.fg_muted)
                    .child("用 Ely + GPUI 构建的原生 Linux AppImage 管理器。"),
            )
            .child(
                card("项目信息", colors.border, colors.surface)
                    .child("作者：xingwangzhe")
                    .child("邮箱：xingwangzhe@outlook.com")
                    .child("架构：x86_64 / aarch64")
                    .child("更新通过上游 GitHub 发布获取；没有配置更新源时会明确提示。"),
            )
            .child(
                div().child(
                    Button::new("repository", "项目主页")
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
fn card(title: &'static str, border: gpui::Hsla, bg: gpui::Hsla) -> gpui::Div {
    div()
        .p_5()
        .rounded_xl()
        .border_1()
        .border_color(border)
        .bg(bg)
        .flex()
        .flex_col()
        .gap_4()
        .child(
            div()
                .text_sm()
                .font_weight(gpui::FontWeight::SEMIBOLD)
                .child(title),
        )
}
fn info(label: &'static str, value: String, colors: &ely_core::theme::Palette) -> gpui::Div {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(div().text_xs().text_color(colors.fg_muted).child(label))
        .child(div().text_sm().child(value))
}
impl Render for ManagerView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors.clone();
        let page = match self.page {
            Page::Library => self.detail(cx),
            Page::Settings => self.settings(cx).into_any_element(),
            Page::About => self.about(cx).into_any_element(),
        };
        FocusScope::new(&self.focus).root().size_full().child(
            div()
                .size_full()
                .flex()
                .flex_col()
                .bg(colors.bg)
                .text_color(colors.fg)
                .text_sm()
                .on_action(cx.listener(|this, _: &Refresh, _, cx| this.refresh(cx)))
                .on_action(
                    cx.listener(|this, _: &Import, window, cx| this.choose_import(window, cx)),
                )
                .on_action(cx.listener(|this, _: &Settings, _, cx| {
                    this.page = Page::Settings;
                    cx.notify();
                }))
                .on_action(cx.listener(|this, _: &About, _, cx| {
                    this.page = Page::About;
                    cx.notify();
                }))
                .on_action(|_: &ToggleTheme, _, cx| {
                    let next = if cx.theme().is_dark() {
                        Mode::Light
                    } else {
                        Mode::Dark
                    };
                    Theme::set_mode(next, cx);
                })
                .on_action(|_: &Quit, _, cx| cx.quit())
                .child(
                    div()
                        .bg(colors.surface)
                        .border_b_1()
                        .border_color(colors.border)
                        .child(MenuBar::new("menu-bar", cx.get_menus().unwrap_or_default())),
                )
                .child(self.topbar(cx))
                .child(
                    div()
                        .flex()
                        .flex_1()
                        .min_h_0()
                        .child(self.sidebar(cx))
                        .child(
                            div()
                                .id("detail-scroll")
                                .flex_1()
                                .min_w_0()
                                .h_full()
                                .overflow_y_scroll()
                                .p_8()
                                .child(page),
                        ),
                )
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
                        .child(
                            div()
                                .text_xs()
                                .text_color(colors.fg_muted)
                                .child(if self.busy {
                                    "处理中…".to_owned()
                                } else {
                                    format!(
                                        "{} 个应用 · {}",
                                        self.apps.len(),
                                        std::env::consts::ARCH
                                    )
                                }),
                        ),
                ),
        )
    }
}
pub fn run() {
    gpui_platform::application()
        .with_assets(Assets)
        .run(|cx: &mut App| {
            ely_core::init(cx).expect("Ely initialization failed");
            Theme::set_mode_now(Mode::Light, cx);
            cx.bind_keys([
                KeyBinding::new("ctrl-r", Refresh, None),
                KeyBinding::new("ctrl-o", Import, None),
                KeyBinding::new("ctrl-,", Settings, None),
                KeyBinding::new("ctrl-q", Quit, None),
            ]);
            cx.set_menus(vec![
                Menu {
                    disabled: false,
                    name: "文件".into(),
                    items: vec![
                        MenuItem::action("导入 AppImage", Import),
                        MenuItem::action("刷新应用列表", Refresh),
                        MenuItem::separator(),
                        MenuItem::action("退出", Quit),
                    ],
                },
                Menu {
                    disabled: false,
                    name: "工具".into(),
                    items: vec![
                        MenuItem::action("设置", Settings),
                        MenuItem::action("切换明暗主题", ToggleTheme),
                    ],
                },
                Menu {
                    disabled: false,
                    name: "帮助".into(),
                    items: vec![MenuItem::action("关于 AppimageMannger", About)],
                },
            ]);
            let bounds = Bounds::centered(None, size(px(1160.), px(800.)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some(SharedString::from("AppimageMannger 管理器")),
                        ..Default::default()
                    }),
                    window_min_size: Some(size(px(880.), px(580.))),
                    app_id: Some("io.github.xingwangzhe.AppimageMannger".into()),
                    ..Default::default()
                },
                |window, cx| cx.new(|cx| ManagerView::new(window, cx)),
            )
            .expect("Failed to open window");
            cx.activate(true);
        });
}
