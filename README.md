# AppImage Manager

[![Linux CI](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml/badge.svg)](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml)

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Français](README.fr.md) · [Español](README.es.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [Italiano](README.it.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [Bahasa Indonesia](README.id.md) · [ไทย](README.th.md) · [العربية](README.ar.md) · [हिन्दी](README.hi.md) · [বাংলা](README.bn.md) · [فارسی](README.fa.md) · [עברית](README.he.md) · [Svenska](README.sv.md) · [Čeština](README.cs.md) · [Română](README.ro.md)

AppImage Manager is a native Linux manager for AppImage applications. Select an application in the left list to see its version, architecture, size, icon and installation path on the right. Import copies an application into the managed directory; it does not remove the original download.

## Three native interfaces

All screenshots show the English interface of version 1.0.1. Every translated README uses the same three screenshots.

### Ely / GPUI

![Ely / GPUI — English interface](docs/images/ely-gpui-en.png)

### Qt 6 Widgets

![Qt 6 Widgets — English interface](docs/images/qt-en.png)

### GNOME / GTK 4 / libadwaita

![GNOME / GTK 4 — English interface](docs/images/gnome-en.png)

## Features

- Import replaces older versions; original downloads remain, and previous installations can be restored from Trash.
- Application icons and AppImage Open With integration; `--repair-registration` restores registration.
- Anonymous HTTPS Release downloads with architecture and SHA-256 checks, real progress, percentage and transferred size. No `gh` or token is required.
- Draggable dividers and saved preferences across Ely/GPUI, Qt 6 Widgets and GNOME/GTK 4, with display names localized into 26 languages.
- Read application metadata and icons without running or mounting the AppImage.

## Install and open

Download the package for your processor from Releases. Choose one installation method below. For the portable method, replace AppimageMannger.AppImage with the downloaded filename. Arch packages are provided; this guide does not assume an official AUR listing.

```sh
# Debian / Ubuntu
sudo apt install ./appimage-mannger*.deb
# Fedora / RPM
sudo dnf install ./appimage-mannger*.rpm
# Arch Linux
sudo pacman -U ./appimage-mannger*.pkg.tar.zst
# AppImage
chmod +x ./AppimageMannger.AppImage
./AppimageMannger.AppImage
```

## Settings that remain saved

The default directory is ~/Applications. Change it in Settings; existing applications are not moved automatically. Choose the actual GPUI/Ely, Qt 6 Widgets or GNOME/GTK 4/libadwaita frontend. Changing the frontend saves your preferences and restarts the manager. Theme, directory, language, selected application and window state are retained. Open Settings → Language to choose one of 26 languages. Error, Info and Warning diagnostic levels deliberately remain in English.

## Updates and removal

Before checking updates, confirm the application’s GitHub repository. The manager reads supported .upd_info metadata or accepts owner/repository manually. Check for a stable release, then install its matching AppImage. Downloads are size and format checked; SHA-256 is verified when a digest is published. Replacement is atomic and the previous version is backed up. A failed update preserves the installed application.

Uninstall moves the application and its desktop entries to Trash, where they can be restored. Personal configuration remains available.


## Development

[BUILD-AND-RELEASE](docs/BUILD-AND-RELEASE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [i18n](docs/i18n/COVERAGE.md) · [1.0.3](docs/RELEASE-1.0.3.md) · [THIRD-PARTY-NOTICES](docs/THIRD-PARTY-NOTICES.md)

```sh
bun run scripts/check-i18n.ts
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
cargo build --release --locked
bun run scripts/package.ts --formats deb
```

## License and source

The project uses AGPL-3.0-only. You may use, modify and redistribute it under that license. Distribution and modified network services carry corresponding-source obligations; consult LICENSE for the full terms.

[Source and releases](https://github.com/xingwangzhe/AppimageMannger)

AGPL-3.0-only © 2026 [xingwangzhe](https://github.com/xingwangzhe) · [xingwangzhe@outlook.com](mailto:xingwangzhe@outlook.com) · [xingwangzhe.fun](https://xingwangzhe.fun) · [LICENSE](LICENSE)
