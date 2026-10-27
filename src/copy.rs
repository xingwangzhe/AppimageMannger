//! Clipboard affordances shared by the GPUI presentation layer.
use crate::i18n::tr;
use ely_core::{
    buttons::{Button as ElyButton, ButtonVariant},
    menus::{ContextMenu, Menu, MenuItem},
    primitives::{IconName, Image},
    typography::SelectableText,
};
use gpui::{
    App, ClickEvent, ClipboardItem, ElementId, IntoElement, ParentElement, RenderOnce,
    SharedString, Styled, Window, div, px,
};
use image::ImageEncoder;
use std::{
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

#[track_caller]
pub fn text(value: impl Into<SharedString>) -> SelectableText {
    let caller = std::panic::Location::caller();
    SelectableText::new(
        SharedString::from(format!(
            "{}:{}:{}",
            caller.file(),
            caller.line(),
            caller.column()
        )),
        value,
    )
}

/// Preserve the normal Ely action while allowing its label to be copied safely.
#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    label: SharedString,
    inner: ElyButton,
}
impl Button {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        let (id, label) = (id.into(), label.into());
        Self {
            inner: ElyButton::new((id.clone(), "action"), label.clone()),
            id,
            label,
        }
    }
    pub fn icon(mut self, icon: IconName) -> Self {
        self.inner = self.inner.icon(icon);
        self
    }
    pub fn name(mut self, name: impl Into<SharedString>) -> Self {
        self.inner = self.inner.name(name);
        self
    }
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.inner = self.inner.variant(variant);
        self
    }
    pub fn primary(self) -> Self {
        self.variant(ButtonVariant::Primary)
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.inner = self.inner.disabled(disabled);
        self
    }
    pub fn full_width(mut self) -> Self {
        self.inner = self.inner.full_width();
        self
    }
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.inner = self.inner.on_click(handler);
        self
    }
}
impl RenderOnce for Button {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let label = self.label;
        ContextMenu::new(
            self.id,
            Menu::new().item(MenuItem::new(tr("复制文字")).on_click(move |_, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(label.to_string()));
            })),
        )
        .child(self.inner)
    }
}

pub fn image(id: impl Into<ElementId>, path: PathBuf, edge: f32) -> impl IntoElement {
    let id = id.into();
    let source = path.clone();
    ContextMenu::new(
        id.clone(),
        Menu::new().item(MenuItem::new(tr("复制图片")).on_click(move |_, cx| {
            let (path, renderer) = (path.clone(), cx.svg_renderer());
            std::thread::spawn(move || {
                if let Err(error) = copy_image(&path, &renderer) {
                    log::warn!("Cannot copy image: {error:#}");
                }
            });
        })),
    )
    .child(
        Image::new((id, "image"), source)
            .fit(gpui::ObjectFit::Contain)
            .size(px(edge))
            .rounded_lg(),
    )
}

fn copy_image(path: &Path, renderer: &gpui::SvgRenderer) -> anyhow::Result<()> {
    let mut bytes = Vec::new();
    std::fs::File::open(path)?
        .take(4 * 1024 * 1024 + 1)
        .read_to_end(&mut bytes)?;
    let format = crate::metadata::icon_extension(&bytes)
        .ok_or_else(|| anyhow::anyhow!("Invalid clipboard image"))?;
    if format == "svg" {
        let svg = renderer.parse_svg(&bytes)?;
        let rendered = renderer.render_parsed(
            &svg,
            gpui::SvgSize::Size(gpui::size(1024.into(), 1024.into())),
        )?;
        let size = rendered.size(0);
        let mut rgba = rendered.as_bytes(0).unwrap_or_default().to_vec();
        for pixel in rgba.as_chunks_mut::<4>().0 {
            pixel.swap(0, 2);
        }
        bytes.clear();
        image::codecs::png::PngEncoder::new(&mut bytes).write_image(
            &rgba,
            size.width.0 as u32,
            size.height.0 as u32,
            image::ExtendedColorType::Rgba8,
        )?;
    }
    // GPUI 0.3.10's Linux clipboard writer advertises text only, even for an
    // Image entry. Use native clipboard owners to publish interoperable PNG.
    let mut command = if std::env::var_os("DISPLAY").is_some() {
        let mut command = Command::new("xclip");
        command.args(["-selection", "clipboard", "-t", "image/png", "-i"]);
        command
    } else {
        let mut command = Command::new("wl-copy");
        command.args(["--type", "image/png"]);
        command
    };
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    let write = child
        .stdin
        .take()
        .ok_or_else(|| anyhow::anyhow!("Missing clipboard pipe"))?
        .write_all(&bytes);
    let started = std::time::Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if started.elapsed() >= std::time::Duration::from_secs(10) {
            let _ = child.kill();
            let _ = child.wait();
            anyhow::bail!("Clipboard image owner timed out");
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    };
    write?;
    anyhow::ensure!(status.success(), "Clipboard image owner failed");
    Ok(())
}

pub fn heading(title: String) -> impl IntoElement {
    div()
        .font_weight(gpui::FontWeight::SEMIBOLD)
        .child(SelectableText::new(
            (
                ElementId::Name(SharedString::from(title.clone())),
                "heading",
            ),
            title,
        ))
}
