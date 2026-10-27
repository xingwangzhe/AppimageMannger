//! GPUI's chrome comes from Ely; Qt and GNOME have real toolkit windows.
use crate::manager::WindowStyle;
use ely_core::shell::TitleBar;
use gpui::{
    AnyElement, App, InteractiveElement, IntoElement, MouseButton, ParentElement, Window, div,
};
pub fn chrome(_style: WindowStyle, _window: &mut Window, _cx: &mut App) -> AnyElement {
    TitleBar::new("gpui-title-bar")
        .child(
            div()
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .child(crate::copy::text(crate::i18n::tr("AppimageMannger 管理器"))),
        )
        .into_any_element()
}
