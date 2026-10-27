# AppImage 管理員

[![Linux CI](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml/badge.svg)](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml)

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Français](README.fr.md) · [Español](README.es.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [Italiano](README.it.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [Bahasa Indonesia](README.id.md) · [ไทย](README.th.md) · [العربية](README.ar.md) · [हिन्दी](README.hi.md) · [বাংলা](README.bn.md) · [فارسی](README.fa.md) · [עברית](README.he.md) · [Svenska](README.sv.md) · [Čeština](README.cs.md) · [Română](README.ro.md)

AppImage 管理員是原生 Linux AppImage 管理員。在左側選取應用程式，右側可查看圖示、版本、架構、大小與安裝位置。匯入會把應用程式複製至管理目錄，不會刪除原始下載檔案。詳細資料頁提供啟動、開啟目錄、修復桌面選單項目和更新操作。

## 三種原生介面

截圖統一使用 1.0.1 的英文介面。所有語言版本的 README 共用以下三張截圖。

### Ely / GPUI

![Ely / GPUI — English interface](docs/images/ely-gpui-en.png)

### Qt 6 Widgets

![Qt 6 Widgets — English interface](docs/images/qt-en.png)

### GNOME / GTK 4 / libadwaita

![GNOME / GTK 4 — English interface](docs/images/gnome-en.png)

## 主要功能

- 匯入新版本時覆蓋已安裝版本；保留原始下載檔，舊版可從垃圾桶復原。
- 應用程式圖示與 AppImage「開啟方式」整合；可用 `--repair-registration` 修復註冊。
- 匿名 HTTPS Release 直接下載，檢查架構與 SHA-256，顯示實際進度、百分比及已下載大小，無需 `gh` 或 token。
- Ely/GPUI、Qt 6 Widgets、GNOME/GTK 4 均支援拖動分隔線及儲存偏好，應用程式名稱提供 26 種語言。
- 讀取中繼資料與圖示時不會執行或掛載 AppImage。

## 安裝與啟動

從 Releases 下載適合處理器的套件，選擇下列其中一種安裝方式。執行可攜版本時，請將 AppimageMannger.AppImage 換成實際下載的檔名。專案提供 Arch 套件；本指南不假設已有官方 AUR 項目。

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

## 持續保存的設定

預設目錄為 ~/Applications，可在設定中變更；原有應用程式不會自動搬移。可選擇實際的 GPUI/Ely、Qt 6 Widgets 或 GNOME/GTK 4/libadwaita 前端。切換後端時，程式先儲存設定，再重新啟動管理員。佈景主題、目錄、語言、選取項目與視窗狀態皆會保留。在「設定 → 語言」選擇 26 種語言之一。診斷等級 Error、Info、Warning 刻意維持英文。

## 更新與移除

檢查更新前，請確認應用程式的 GitHub 儲存庫。程式會讀取支援的 .upd_info 中繼資料，也可手動輸入 owner/repository。找到穩定版本後，安裝符合本機架構的 AppImage。下載會檢查大小與格式；發行者提供摘要時會驗證 SHA-256。替換採原子操作並備份舊版本，失敗時保留已安裝的應用程式。

解除安裝會把應用程式與對應桌面項目移至垃圾桶，可從垃圾桶復原。個人設定仍然保留。變更管理目錄僅影響掃描與之後的匯入，不會移動舊檔。


## 開發

[BUILD-AND-RELEASE](docs/BUILD-AND-RELEASE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [i18n](docs/i18n/COVERAGE.md) · [1.0.2](docs/RELEASE-1.0.2.md) · [THIRD-PARTY-NOTICES](docs/THIRD-PARTY-NOTICES.md)

```sh
bun run scripts/check-i18n.ts
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
cargo build --release --locked
bun run scripts/package.ts --formats deb
```

## 授權與原始碼

本專案採 AGPL-3.0-only。可依授權使用、修改與散布；散布或提供修改後的網路服務時，應依授權提供對應原始碼。完整條款請見 LICENSE。

[原始碼與發行套件](https://github.com/xingwangzhe/AppimageMannger)

AGPL-3.0-only © 2026 [xingwangzhe](https://github.com/xingwangzhe) · [xingwangzhe@outlook.com](mailto:xingwangzhe@outlook.com) · [xingwangzhe.fun](https://xingwangzhe.fun) · [LICENSE](LICENSE)
