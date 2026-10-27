# AppimageMannger: bắt đầu nhanh

AppimageMannger là trình quản lý AppImage gốc dành cho Linux. Chọn ứng dụng trong danh sách bên trái để xem biểu tượng, phiên bản, kiến trúc, dung lượng và đường dẫn cài đặt ở bên phải. Nhập ứng dụng sẽ sao chép tệp vào thư mục quản lý và giữ lại bản tải xuống ban đầu.

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

## Giấy phép và mã nguồn

Dự án dùng AGPL-3.0-only. Việc dùng, sửa đổi và phân phối tuân theo giấy phép này. Phân phối hoặc cung cấp dịch vụ mạng đã sửa đổi có nghĩa vụ cung cấp mã nguồn tương ứng. Xem đầy đủ trong LICENSE.

[Mã nguồn và bản phát hành](https://github.com/xingwangzhe/AppimageMannger)
