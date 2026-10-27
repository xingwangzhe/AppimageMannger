//! GPUI's chrome comes from Ely; Qt and GNOME have real toolkit windows.
use crate::manager::WindowStyle;
use ely_core::shell::TitleBar;
use gpui::{AnyElement, App, IntoElement, Window};
pub fn chrome(_style: WindowStyle, _window: &mut Window, _cx: &mut App) -> AnyElement {
    TitleBar::new("gpui-title-bar")
        .title(crate::i18n::tr("AppimageMannger 管理器"))
        .into_any_element()
}
