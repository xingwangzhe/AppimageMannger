# AppimageMannger：クイックスタート

AppimageMannger は Linux 用のネイティブな AppImage 管理アプリです。左の一覧でアプリを選ぶと、右側にアイコン、バージョン、アーキテクチャ、サイズ、保存場所が表示されます。インポートは管理フォルダーへのコピーであり、ダウンロード元のファイルは削除しません。

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

## ライセンスとソース

AGPL-3.0-only に従って利用、変更、再配布できます。配布や変更版のネットワーク提供には対応するソースの提供義務があります。完全な条件は LICENSE を参照してください。

[ソースとリリース](https://github.com/xingwangzhe/AppimageMannger)
