# Trình quản lý AppImage

[![Linux CI](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml/badge.svg)](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml)

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Français](README.fr.md) · [Español](README.es.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [Italiano](README.it.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [Bahasa Indonesia](README.id.md) · [ไทย](README.th.md) · [العربية](README.ar.md) · [हिन्दी](README.hi.md) · [বাংলা](README.bn.md) · [فارسی](README.fa.md) · [עברית](README.he.md) · [Svenska](README.sv.md) · [Čeština](README.cs.md) · [Română](README.ro.md)

Trình quản lý AppImage là trình quản lý AppImage gốc dành cho Linux. Chọn ứng dụng trong danh sách bên trái để xem biểu tượng, phiên bản, kiến trúc, dung lượng và đường dẫn cài đặt ở bên phải. Nhập ứng dụng sẽ sao chép tệp vào thư mục quản lý và giữ lại bản tải xuống ban đầu.

## Ba giao diện gốc

Tất cả ảnh chụp đều dùng giao diện tiếng Anh của phiên bản 1.0.1. Các README đã dịch dùng chung ba ảnh này.

### Ely / GPUI

![Ely / GPUI — English interface](docs/images/ely-gpui-en.png)

### Qt 6 Widgets

![Qt 6 Widgets — English interface](docs/images/qt-en.png)

### GNOME / GTK 4 / libadwaita

![GNOME / GTK 4 — English interface](docs/images/gnome-en.png)

## Tính năng

- Nhập phiên bản mới sẽ thay phiên bản cũ. Tệp tải gốc được giữ lại; bản cũ có thể khôi phục từ thùng rác.
- Biểu tượng ứng dụng và tích hợp AppImage trong “Mở bằng”. `--repair-registration` sửa đăng ký.
- Tải trực tiếp Release qua HTTPS ẩn danh, kiểm tra kiến trúc và SHA-256, hiển thị tiến độ thực, phần trăm và dung lượng đã tải. Không cần `gh` hoặc token.
- Kéo đường chia và lưu tùy chọn trong Ely/GPUI, Qt 6 Widgets và GNOME/GTK 4; tên hiển thị được dịch sang 26 ngôn ngữ.
- Đọc siêu dữ liệu và biểu tượng mà không chạy hay gắn kết AppImage.

## Cài đặt và mở

Tải gói phù hợp với bộ xử lý từ Releases rồi chọn một cách dưới đây. Với bản di động, thay AppimageMannger.AppImage bằng tên tệp thực tế. Có gói Arch; hướng dẫn không giả định dự án đã có mục AUR chính thức.

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

## Thiết lập được lưu

Thư mục mặc định là ~/Applications. Có thể đổi trong Cài đặt; ứng dụng hiện có không tự chuyển sang thư mục mới. Chọn giao diện thật GPUI/Ely, Qt 6 Widgets hoặc GNOME/GTK 4/libadwaita. Khi đổi, chương trình lưu thiết lập rồi khởi động lại trình quản lý. Chủ đề, thư mục, ngôn ngữ, ứng dụng đã chọn và trạng thái cửa sổ được giữ lại. Cài đặt → Ngôn ngữ cung cấp 26 ngôn ngữ. Các mức chẩn đoán Error, Info và Warning cố ý giữ nguyên tiếng Anh.

## Cập nhật và gỡ bỏ

Trước khi kiểm tra cập nhật, xác nhận kho GitHub của ứng dụng. Chương trình đọc .upd_info được hỗ trợ hoặc cho nhập owner/repository thủ công. Tìm bản ổn định rồi cài AppImage đúng kiến trúc. Kích thước và định dạng được kiểm tra; SHA-256 được xác minh nếu có mã băm công bố. Việc thay thế có tính nguyên tử và giữ bản sao phiên bản cũ. Cập nhật thất bại không làm mất ứng dụng đã cài.

Gỡ bỏ chuyển ứng dụng và mục màn hình vào Thùng rác để có thể khôi phục. Thiết lập cá nhân vẫn được giữ.


## Phát triển

[BUILD-AND-RELEASE](docs/BUILD-AND-RELEASE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [i18n](docs/i18n/COVERAGE.md) · [1.0.3](docs/RELEASE-1.0.3.md) · [THIRD-PARTY-NOTICES](docs/THIRD-PARTY-NOTICES.md)

```sh
bun run scripts/check-i18n.ts
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
cargo build --release --locked
bun run scripts/package.ts --formats deb
```

## Giấy phép và mã nguồn

Dự án dùng AGPL-3.0-only. Việc dùng, sửa đổi và phân phối tuân theo giấy phép này. Phân phối hoặc cung cấp dịch vụ mạng đã sửa đổi có nghĩa vụ cung cấp mã nguồn tương ứng. Xem đầy đủ trong LICENSE.

[Mã nguồn và bản phát hành](https://github.com/xingwangzhe/AppimageMannger)

AGPL-3.0-only © 2026 [xingwangzhe](https://github.com/xingwangzhe) · [xingwangzhe@outlook.com](mailto:xingwangzhe@outlook.com) · [xingwangzhe.fun](https://xingwangzhe.fun) · [LICENSE](LICENSE)
