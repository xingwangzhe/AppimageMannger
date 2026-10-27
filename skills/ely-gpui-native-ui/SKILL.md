---
name: ely-gpui-native-ui
description: Build or debug AppimageMannger's Ely/GPUI frontend, including native drag chrome, persistent preferences, shared translations and application icons. Use for this project's GPUI interface; Qt and GTK widgets live in their separate native frontends.
---

Read [AGENTS.md](../../AGENTS.md) and the relevant parts of [architecture](../../docs/ARCHITECTURE.md) first.

Use Ely components where they provide the behavior: TitleBar and WindowControls for drag/close/maximize, ListItem for application rows, Card/CardHeader for groups, Image for icons, Select for language, SegmentedControl for toolkit selection, and the existing input/button/menu components. Raw GPUI remains appropriate for layout, events and native window setup.

`WindowOptions.titlebar` alone does not draw a draggable Linux caption. Keep a real Ely TitleBar above interactive menus/toolbars. Put action handlers outside the root FocusScope so actions still reach them when the scope itself holds focus. Do not make the full content area draggable.

Preferences come from `manager::Config`. Load before applying theme/window options. Save toolkit/language before restarting through the engine; Qt/GNOME modes are real toolkit processes. Preserve drafts independently from validated applied values. Keep window bounds observers and Ely `guard_close` aligned with persistence.

Resolve icons during scan with `metadata::resolve_icon`, then display `AppInfo.icon_path`; do not spawn extractors in render or permanently substitute a package icon for an existing application icon.

Visible strings use `i18n::tr` and the shared catalogs. Preserve placeholders and English diagnostic levels. Use `i18n::ely_i18n` for Ely's exact locale tags and localized Browse/Choose overrides; unsupported built-in component copy safely falls back to English. Passing arbitrary application locale codes directly to Ely can panic. Check RTL and a long-label language when changing layout.

Validate with focused Rust checks and a real local window: drag the caption, operate its controls, switch theme/language/toolkit, restart, and confirm saved values and icon rendering. Screenshots are evidence of rendering, not proof that persistence or update operations succeeded. See [build and release](../../docs/BUILD-AND-RELEASE.md).
