# AppImage 管理器

[![Linux CI](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml/badge.svg)](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml)

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Français](README.fr.md) · [Español](README.es.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [Italiano](README.it.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [Bahasa Indonesia](README.id.md) · [ไทย](README.th.md) · [العربية](README.ar.md) · [हिन्दी](README.hi.md) · [বাংলা](README.bn.md) · [فارسی](README.fa.md) · [עברית](README.he.md) · [Svenska](README.sv.md) · [Čeština](README.cs.md) · [Română](README.ro.md)

AppImage 管理器是原生 Linux AppImage 管理器。在左侧选择应用，右侧查看图标、版本、架构、大小和安装位置。导入会将应用复制到管理目录，不会删除原始下载文件。可以从详情页启动应用、打开目录、修复桌面菜单入口或管理更新。

## 三种原生界面

截图统一使用 1.0.1 的英文界面。所有语言版本的 README 共用以下三张截图。

### Ely / GPUI

![Ely / GPUI — English interface](docs/images/ely-gpui-en.png)

### Qt 6 Widgets

![Qt 6 Widgets — English interface](docs/images/qt-en.png)

### GNOME / GTK 4 / libadwaita

![GNOME / GTK 4 — English interface](docs/images/gnome-en.png)

## 主要功能

- 导入新版本时覆盖已安装版本；保留原始下载文件，旧版可从回收站恢复。
- 应用图标和 AppImage“打开方式”集成；可用 `--repair-registration` 修复注册。
- 匿名 HTTPS Release 直接下载，检查架构和 SHA-256，显示真实进度、百分比及已下载大小，无需 `gh` 或 token。
- Ely/GPUI、Qt 6 Widgets、GNOME/GTK 4 均支持拖动分隔线和保存偏好，应用显示名称覆盖 26 种语言。
- 读取应用元数据和图标时不会运行或挂载 AppImage。

## 安装与启动

从 Releases 下载适合本机处理器的安装包，下列方式选择一种。使用便携版本时，请把 AppimageMannger.AppImage 替换为实际下载的文件名。提供 Arch 安装包；此指南不假设项目已经发布到官方 AUR。

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

## 会保存的设置

默认应用目录是 ~/Applications，可在设置中更改；已有应用不会自动迁移。可选择真正的 GPUI/Ely、Qt 6 Widgets 或 GNOME/GTK 4/libadwaita 前端。切换后端时，程序先保存设置，再重启管理器。主题、目录、语言、选中的应用及窗口状态都会保留。在“设置 → 语言”选择 26 种语言之一。诊断级别 Error、Info、Warning 特意保持英文。

## 更新与卸载

检查更新前，先核对应用的 GitHub 仓库。程序会读取支持的 .upd_info 元数据，也可手动填写 owner/repository。检查稳定版本后，安装匹配本机架构的 AppImage。下载会检查大小和格式；发布方提供摘要时还会校验 SHA-256。更新采用原子替换并备份旧版本，失败时保留已安装的应用。

卸载将应用和对应桌面入口移入回收站，可从回收站恢复。个人配置不会一并删除。修改管理目录只改变扫描和后续导入的位置，不会移动旧文件。


## 开发

[BUILD-AND-RELEASE](docs/BUILD-AND-RELEASE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [i18n](docs/i18n/COVERAGE.md) · [1.0.1](docs/RELEASE-1.0.1.md) · [THIRD-PARTY-NOTICES](docs/THIRD-PARTY-NOTICES.md)

```sh
bun run scripts/check-i18n.ts
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
cargo build --release --locked
bun run scripts/package.ts --formats deb
```

## 协议与源码

项目采用 AGPL-3.0-only。可依照协议使用、修改和分发；分发或提供修改后的网络服务时，须按协议提供相应源码。完整条款见 LICENSE。

[源码与发行包](https://github.com/xingwangzhe/AppimageMannger)

AGPL-3.0-only © 2026 xingwangzhe · xingwangzhe@outlook.com · [LICENSE](LICENSE)
