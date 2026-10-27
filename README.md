# AppimageMannger

[![Linux CI](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml/badge.svg)](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml)

A Linux AppImage manager with a shared Rust engine and three **real native frontends**: Ely/GPUI, Qt 6 Widgets, and GTK 4/libadwaita. Project license: **AGPL-3.0-only**.

The application and user guides support 26 languages. Choose a guide below, or change **Settings → Language** in the application. Toolkit, language and theme choices are saved and restored on restart. Diagnostic levels and engine errors remain English.

[English](docs/i18n/en.md) · [简体中文](docs/i18n/zh-CN.md) · [繁體中文](docs/i18n/zh-TW.md) · [日本語](docs/i18n/ja.md) · [한국어](docs/i18n/ko.md) · [Deutsch](docs/i18n/de.md) · [Français](docs/i18n/fr.md) · [Español](docs/i18n/es.md) · [Português (Brasil)](docs/i18n/pt-BR.md) · [Русский](docs/i18n/ru.md) · [Italiano](docs/i18n/it.md) · [Nederlands](docs/i18n/nl.md) · [Polski](docs/i18n/pl.md) · [Türkçe](docs/i18n/tr.md) · [Українська](docs/i18n/uk.md) · [Tiếng Việt](docs/i18n/vi.md) · [Bahasa Indonesia](docs/i18n/id.md) · [ไทย](docs/i18n/th.md) · [العربية](docs/i18n/ar.md) · [हिन्दी](docs/i18n/hi.md) · [বাংলা](docs/i18n/bn.md) · [فارسی](docs/i18n/fa.md) · [עברית](docs/i18n/he.md) · [Svenska](docs/i18n/sv.md) · [Čeština](docs/i18n/cs.md) · [Română](docs/i18n/ro.md)

## Features

- Searchable application list and details: real icon, version, size, architecture and installation path.
- Default library: `~/Applications`, configurable without moving existing files.
- Import AppImages, create/repair desktop menu entries, launch applications and open their directory.
- Importing another version replaces the existing application and retains a restorable Trash backup. Right-click an AppImage in your file manager and choose the localized AppImage Manager under Open With to import it directly.
- Discover GitHub stable updates from embedded metadata or an explicitly configured official repository; install an architecture-matching AppImage with size/format/digest checks, atomic replacement and rollback data.
- Recoverable uninstall through freedesktop Trash, preserving personal application data.
- Real toolkit switching: **GPUI** uses Ely components; **Qt** loads Qt 6; **GNOME** loads GTK 4/libadwaita. Switching toolkit or language saves first and restarts the application.
- Persistent language, theme, toolkit, directory/repository settings, editing drafts, selected application, window size and maximized state.
- Draggable, persistent library/detail divider and application display names localized in all 26 languages.
- AppImage metadata/icon inspection never launches or mounts the application.

## Install

Download a matching architecture package from [Releases](https://github.com/xingwangzhe/AppimageMannger/releases).

```sh
# Debian / Ubuntu
sudo apt install ./appimage-mannger_*_amd64.deb
# Fedora / RPM systems
sudo dnf install ./appimage-mannger-*.x86_64.rpm
# Arch / pacman systems
sudo pacman -U ./appimage-mannger-*.pkg.tar.zst
# Standalone AppImage
chmod +x AppimageMannger-*.AppImage
./AppimageMannger-*.AppImage
```

ARM64 packages use `arm64` / `aarch64` filenames. AUR PKGBUILD/.SRCINFO archives are also provided as recipes; they do not require an AUR account to use and are separate from directly installable pacman packages. No AUR service publication is implied.

Current CI uses native Ubuntu 26.04 x86_64/aarch64 runners, current stable Rust and current Bun. Packages built on a recent system may require recent glibc/toolkit versions; AppImage does not bundle libc or GPU drivers. Inspect dependency/build-info files for the actual baseline.

## Use

Open an application on the left to view its details. If its update source is unknown, enter the upstream `owner/repository`, save it, and check for updates. Unsupported version metadata, missing architecture assets and network failures produce explicit diagnostics. A newer matching asset enables installation.

Update discovery reads public HTTPS Release pages and downloads release attachments directly. Clients need neither the `gh` command nor a GitHub token. `appimage-mannger --repair-registration` restores the file manager's Open With registration if necessary.

Use the window caption to drag or maximize the window. Settings provide the persistent native toolkit, language, light/dark appearance and application library. Directory and repository edits are retained as drafts until applied; existing files are not moved when selecting another library.

Shortcuts in the Ely/GPUI frontend: `Ctrl+O` import, `Ctrl+R` refresh, `Ctrl+,` settings, `Ctrl+Q` quit. Other toolkits use their native menus and controls.

## Develop and verify

See [build and release](docs/BUILD-AND-RELEASE.md), [architecture](docs/ARCHITECTURE.md), [agent guidance](AGENTS.md), and [repository skills](skills/).

```sh
bun run scripts/check-i18n.ts
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
cargo build --release --locked
bun run scripts/package.ts --formats deb
codegraph init -y .
codegraph sync .
```

Native development requires Qt6Widgets, GTK4, libadwaita and json-glib alongside the GPUI platform libraries. `build.rs` builds both helpers and generates their English fallback from the same catalogs used by Rust. All frontends call the same bounded JSON RPC engine; no shell interpolation is used for paths or requests.

[Translation coverage](docs/i18n/COVERAGE.md) records the actual catalog/core/placeholder checks. [1.0.0 handoff](docs/RELEASE-1.0.0.md) records verified local and CI results. [Third-party notices](docs/THIRD-PARTY-NOTICES.md) explain dependency licensing; the project's AGPL choice does not replace Qt, GTK or Ely licenses.

## License

AGPL-3.0-only © 2026 xingwangzhe · xingwangzhe@outlook.com. See [LICENSE](LICENSE).
