# AppimageMannger: panduan cepat

AppimageMannger adalah pengelola aplikasi AppImage asli untuk Linux. Pilih aplikasi pada daftar kiri untuk melihat ikon, versi, arsitektur, ukuran, dan lokasi pemasangannya di kanan. Impor menyalin berkas ke folder terkelola dan tidak menghapus unduhan awal. Halaman detail juga menyediakan peluncuran dan perbaikan entri menu desktop.

## Memasang dan membuka

Unduh paket yang sesuai dengan prosesor dari Releases, lalu pilih salah satu cara berikut. Untuk versi portabel, ganti AppimageMannger.AppImage dengan nama berkas sebenarnya. Paket Arch tersedia; panduan ini tidak menganggap adanya entri AUR resmi.

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

## Pengaturan yang tersimpan

Folder bawaan adalah ~/Applications. Ubah melalui Pengaturan; aplikasi lama tidak otomatis dipindahkan. Pilih antarmuka nyata GPUI/Ely, Qt 6 Widgets, atau GNOME/GTK 4/libadwaita. Pergantian menyimpan preferensi dan memulai ulang pengelola. Tema, folder, bahasa, aplikasi pilihan, dan keadaan jendela dipertahankan. Pengaturan → Bahasa menyediakan 26 bahasa. Tingkat diagnostik Error, Info, dan Warning sengaja tetap dalam bahasa Inggris.

## Pembaruan dan penghapusan

Sebelum memeriksa pembaruan, pastikan repositori GitHub aplikasi benar. Metadata .upd_info yang didukung dibaca, atau owner/repository dapat diisi secara manual. Cari rilis stabil lalu pasang AppImage yang sesuai. Ukuran dan format diperiksa; SHA-256 diverifikasi bila digest diterbitkan. Penggantian bersifat atomik dan versi sebelumnya dicadangkan. Pembaruan gagal tidak merusak aplikasi terpasang.

Pencopotan memindahkan aplikasi dan entri desktop ke Sampah agar dapat dipulihkan. Pengaturan pribadi tetap disimpan. Mengubah folder hanya memengaruhi pemindaian dan impor berikutnya.

## Lisensi dan sumber

Proyek menggunakan AGPL-3.0-only. Penggunaan, perubahan, dan distribusi mengikuti lisensi ini. Distribusi serta layanan jaringan yang dimodifikasi membawa kewajiban penyediaan kode sumber terkait. Baca LICENSE untuk ketentuan lengkap.

[Sumber dan rilis](https://github.com/xingwangzhe/AppimageMannger)
