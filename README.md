# AppimageMannger 管理器

[![Linux CI](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml/badge.svg)](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml)

使用 [Ely GPUI Components](https://elygpui.com/) 与 Rust 编写的原生 Linux AppImage 管理器。保留项目名称 `AppimageMannger`，命令为 `appimage-mannger`。

## 功能

- 左侧可搜索应用列表，右侧查看版本、大小、架构和安装位置。
- 默认管理 `~/Applications`；设置中可以切换应用目录。
- 导入本地 AppImage，自动创建用户菜单入口；源文件保留。
- 启动应用、打开目录、修复菜单入口。
- 通过内嵌更新信息或手动配置的 GitHub 仓库检查最新稳定版。
- 下载当前架构 AppImage，校验下载大小、ELF/AppImage 格式和上游提供的 SHA-256，原子安装更新并保留旧版。
- 卸载时移至回收站，移除对应菜单入口，保留应用个人数据。
- 左上角文件、工具、帮助菜单；设置、关于与明暗主题。

快捷键：`Ctrl+O` 导入、`Ctrl+R` 刷新、`Ctrl+,` 设置、`Ctrl+Q` 退出。Ely 输入框和按钮支持 Tab 焦点切换。

## 安装

从 [Releases](https://github.com/xingwangzhe/AppimageMannger/releases) 下载匹配架构的包。初始版本为测试预发布版。

```sh
# Debian / Ubuntu
sudo apt install ./appimage-mannger_*_amd64.deb
# Fedora / RPM 系
sudo dnf install ./appimage-mannger-*.x86_64.rpm
# 独立运行
chmod +x AppimageMannger-*.AppImage
./AppimageMannger-*.AppImage
```

ARM64 对应 `arm64` / `aarch64` 文件。需要可用的桌面会话、GPU/Vulkan 驱动和中文字体。AppImage 同样使用宿主 libc 和 GPU 驱动。

CI 使用当前最新 GA Ubuntu 26.04 x86_64 / ARM64 原生 runner、最新 stable Rust、最新 Bun 和发布时解析的官方最新 appimagetool。最新系统构建的二进制可能要求较新的 glibc；deb/rpm 记录依赖，AppImage 不保证兼容旧发行版。构建来源与工具版本见 release 的 build-info。

## 更新源

若应用没有可识别的 GitHub 更新信息，在详情页输入其官方 `owner/repository` 后保存，再点击“检查更新”。软件会明确报告未配置、无新版本、缺少适配架构包或网络错误。只有确认版本较新且找到 AppImage 时才启用“安装更新”。更新过程中请勿自行替换目标文件。未知版本、非 GitHub 更新协议、签名验证和增量 zsync 暂不支持。

GpgFrontend 等已有应用会直接显示在默认目录中；扫描不会运行或修改它们。目录设置不会搬迁已有文件。

## 本地开发与打包

Rust 1.98+（推荐最新 stable）、Bun、C/C++ 工具链和 Linux 图形开发库：

```sh
sudo apt install build-essential pkg-config clang cmake libfontconfig-dev libxkbcommon-x11-dev libwayland-dev libvulkan-dev
cargo build --release --locked
cargo test --locked
cargo clippy --all-targets --locked -- -D warnings
bun run scripts/package.ts --formats deb
```

```sh
appimage-mannger --version
appimage-mannger --list
appimage-mannger --smoke-test --directory /tmp/empty-appimage-library
codegraph init -y .
codegraph sync .
```

完整打包：`bun run scripts/package.ts --formats deb,rpm,appimage,tar`，再运行 `bun run scripts/package-aur.ts` 和 `bun run scripts/package-checksums.ts`。需要 dpkg-dev、rpm、squashfs-tools 等工具；CI 自动安装。每个架构均在对应原生 runner 上编译和打包。发布 tag 必须与 Cargo 版本完全匹配，推送 `v<version>` 后 CI 测试、构建、安装烟雾测试通过才发布。

AUR 产物包含带 SHA-256 的 `PKGBUILD`、`.SRCINFO` 和 recipe 源包，覆盖源代码构建与二进制安装方案。它们作为 release 产物提供；提交到 aur.archlinux.org 需要单独的 AUR 账户与 SSH 权限。

目录与模块职责见 [架构说明](docs/ARCHITECTURE.md)。

## 协议

MIT © 2026 xingwangzhe · xingwangzhe@outlook.com
