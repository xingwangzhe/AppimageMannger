# AppImage マネージャー

[![Linux CI](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml/badge.svg)](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml)

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Français](README.fr.md) · [Español](README.es.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [Italiano](README.it.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [Bahasa Indonesia](README.id.md) · [ไทย](README.th.md) · [العربية](README.ar.md) · [हिन्दी](README.hi.md) · [বাংলা](README.bn.md) · [فارسی](README.fa.md) · [עברית](README.he.md) · [Svenska](README.sv.md) · [Čeština](README.cs.md) · [Română](README.ro.md)

AppImage マネージャー は Linux 用のネイティブな AppImage 管理アプリです。左の一覧でアプリを選ぶと、右側にアイコン、バージョン、アーキテクチャ、サイズ、保存場所が表示されます。インポートは管理フォルダーへのコピーであり、ダウンロード元のファイルは削除しません。

## 3 種類のネイティブ UI

画像はすべてバージョン 1.0.1 の英語 UI です。各言語の README で同じ 3 枚を使用しています。

### Ely / GPUI

![Ely / GPUI — English interface](docs/images/ely-gpui-en.png)

### Qt 6 Widgets

![Qt 6 Widgets — English interface](docs/images/qt-en.png)

### GNOME / GTK 4 / libadwaita

![GNOME / GTK 4 — English interface](docs/images/gnome-en.png)

## 主な機能

- 新しいバージョンのインポートで既存版を置き換えます。元のダウンロードは残り、旧版はゴミ箱から復元できます。
- アプリアイコンと AppImage の「別のアプリで開く」を統合。`--repair-registration` で登録を修復できます。
- 匿名 HTTPS で Release を直接取得。アーキテクチャと SHA-256 を検証し、進捗、割合、転送量を表示します。`gh` やトークンは不要です。
- Ely/GPUI、Qt 6 Widgets、GNOME/GTK 4 で区切り線をドラッグでき、設定を保存します。表示名は 26 言語に対応します。
- AppImage を実行・マウントせずにメタデータとアイコンを読み取ります。

## インストールと起動

Releases から CPU に合うパッケージを取得し、次の方法を一つ選んでください。ポータブル版では AppimageMannger.AppImage を実際のファイル名に置き換えます。Arch パッケージも提供しますが、この説明は公式 AUR 登録を前提としていません。

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

## 保存される設定

既定のフォルダーは ~/Applications です。設定で変更できますが、既存のアプリは自動移動しません。実際の GPUI/Ely、Qt 6 Widgets、GNOME/GTK 4/libadwaita フロントエンドを選べます。切り替えると設定を保存してから管理アプリを再起動します。テーマ、フォルダー、言語、選択したアプリ、ウィンドウ状態は保持されます。「設定 → 言語」で 26 言語から選択してください。診断レベルの Error、Info、Warning は意図的に英語のままです。

## 更新とアンインストール

更新前に、対象アプリの GitHub リポジトリを確認してください。対応する .upd_info を読み取るほか、owner/repository を手動で指定できます。安定版を確認して、CPU に対応する AppImage をインストールします。サイズと形式を確認し、公開されたダイジェストがある場合は SHA-256 も検証します。置き換えはアトミックに行い、旧版をバックアップします。失敗した場合はインストール済みアプリを保護します。

アンインストールではアプリとデスクトップ項目をゴミ箱に移します。そこから復元でき、個人設定は残ります。


## 開発

[BUILD-AND-RELEASE](docs/BUILD-AND-RELEASE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [i18n](docs/i18n/COVERAGE.md) · [1.0.1](docs/RELEASE-1.0.1.md) · [THIRD-PARTY-NOTICES](docs/THIRD-PARTY-NOTICES.md)

```sh
bun run scripts/check-i18n.ts
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
cargo build --release --locked
bun run scripts/package.ts --formats deb
```

## ライセンスとソース

AGPL-3.0-only に従って利用、変更、再配布できます。配布や変更版のネットワーク提供には対応するソースの提供義務があります。完全な条件は LICENSE を参照してください。

[ソースとリリース](https://github.com/xingwangzhe/AppimageMannger)

AGPL-3.0-only © 2026 xingwangzhe · xingwangzhe@outlook.com · [LICENSE](LICENSE)
