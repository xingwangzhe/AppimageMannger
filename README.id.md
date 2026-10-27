# Pengelola AppImage

[![Linux CI](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml/badge.svg)](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml)

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Français](README.fr.md) · [Español](README.es.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [Italiano](README.it.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [Bahasa Indonesia](README.id.md) · [ไทย](README.th.md) · [العربية](README.ar.md) · [हिन्दी](README.hi.md) · [বাংলা](README.bn.md) · [فارسی](README.fa.md) · [עברית](README.he.md) · [Svenska](README.sv.md) · [Čeština](README.cs.md) · [Română](README.ro.md)

Pengelola AppImage adalah pengelola aplikasi AppImage asli untuk Linux. Pilih aplikasi pada daftar kiri untuk melihat ikon, versi, arsitektur, ukuran, dan lokasi pemasangannya di kanan. Impor menyalin berkas ke folder terkelola dan tidak menghapus unduhan awal. Halaman detail juga menyediakan peluncuran dan perbaikan entri menu desktop.

## Tiga antarmuka native

Semua tangkapan layar menggunakan antarmuka bahasa Inggris versi 1.0.1. Setiap README terjemahan memakai tiga gambar yang sama.

### Ely / GPUI

![Ely / GPUI — English interface](docs/images/ely-gpui-en.png)

### Qt 6 Widgets

![Qt 6 Widgets — English interface](docs/images/qt-en.png)

### GNOME / GTK 4 / libadwaita

![GNOME / GTK 4 — English interface](docs/images/gnome-en.png)

## Fitur

- Impor mengganti versi lama. Unduhan asli tetap disimpan; instalasi sebelumnya dapat dipulihkan dari tempat sampah.
- Ikon aplikasi dan integrasi AppImage pada “Buka dengan”. `--repair-registration` memperbaiki pendaftaran.
- Unduh Release langsung melalui HTTPS anonim, dengan pemeriksaan arsitektur dan SHA-256, progres nyata, persentase, serta ukuran transfer. Tidak memerlukan `gh` atau token.
- Pemisah yang dapat diseret dan preferensi tersimpan pada Ely/GPUI, Qt 6 Widgets, serta GNOME/GTK 4; nama tampilan dalam 26 bahasa.
- Membaca metadata dan ikon tanpa menjalankan atau memasang AppImage.

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


## Pengembangan

[BUILD-AND-RELEASE](docs/BUILD-AND-RELEASE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [i18n](docs/i18n/COVERAGE.md) · [1.0.3](docs/RELEASE-1.0.3.md) · [THIRD-PARTY-NOTICES](docs/THIRD-PARTY-NOTICES.md)

```sh
bun run scripts/check-i18n.ts
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
cargo build --release --locked
bun run scripts/package.ts --formats deb
```

## Lisensi dan sumber

Proyek menggunakan AGPL-3.0-only. Penggunaan, perubahan, dan distribusi mengikuti lisensi ini. Distribusi serta layanan jaringan yang dimodifikasi membawa kewajiban penyediaan kode sumber terkait. Baca LICENSE untuk ketentuan lengkap.

[Sumber dan rilis](https://github.com/xingwangzhe/AppimageMannger)

AGPL-3.0-only © 2026 [xingwangzhe](https://github.com/xingwangzhe) · [xingwangzhe@outlook.com](mailto:xingwangzhe@outlook.com) · [xingwangzhe.fun](https://xingwangzhe.fun) · [LICENSE](LICENSE)
