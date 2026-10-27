# مدیر AppImage

[![Linux CI](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml/badge.svg)](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml)

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Français](README.fr.md) · [Español](README.es.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [Italiano](README.it.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [Bahasa Indonesia](README.id.md) · [ไทย](README.th.md) · [العربية](README.ar.md) · [हिन्दी](README.hi.md) · [বাংলা](README.bn.md) · [فارسی](README.fa.md) · [עברית](README.he.md) · [Svenska](README.sv.md) · [Čeština](README.cs.md) · [Română](README.ro.md)

مدیر AppImage یک برنامهٔ بومی Linux برای مدیریت AppImage است. با انتخاب برنامه در کتابخانه، آیکون، نسخه، معماری، اندازه و مسیر نصب نمایش داده می‌شود. وارد کردن فایل را به پوشهٔ مدیریت‌شده کپی می‌کند و دانلود اصلی را نگه می‌دارد.

## سه رابط بومی

همهٔ تصاویر رابط انگلیسی نسخهٔ 1.0.1 را نشان می‌دهند. تمام ترجمه‌های README از همین سه تصویر استفاده می‌کنند.

### Ely / GPUI

![Ely / GPUI — English interface](docs/images/ely-gpui-en.png)

### Qt 6 Widgets

![Qt 6 Widgets — English interface](docs/images/qt-en.png)

### GNOME / GTK 4 / libadwaita

![GNOME / GTK 4 — English interface](docs/images/gnome-en.png)

## امکانات

- وارد کردن نسخهٔ جدید جایگزین نسخهٔ قدیمی می‌شود. فایل اصلی دانلود حفظ می‌شود و نصب قبلی از سطل زباله قابل بازیابی است.
- آیکون برنامه و یکپارچه‌سازی AppImage با «باز کردن با». دستور `--repair-registration` ثبت برنامه را ترمیم می‌کند.
- دانلود مستقیم Release از طریق HTTPS ناشناس، بررسی معماری و SHA-256 و نمایش پیشرفت واقعی، درصد و حجم انتقال. نیازی به `gh` یا توکن نیست.
- جداکننده‌های قابل کشیدن و تنظیمات ماندگار در Ely/GPUI،‏ Qt 6 Widgets و GNOME/GTK 4؛ نام نمایشی به ۲۶ زبان ترجمه شده است.
- خواندن فراداده و آیکون‌ها بدون اجرا یا سوار کردن AppImage.

## نصب و اجرا

از Releases بستهٔ مناسب پردازندهٔ خود را دریافت کنید و یکی از روش‌های زیر را انتخاب کنید. برای نسخهٔ قابل‌حمل، AppimageMannger.AppImage را با نام واقعی فایل جایگزین کنید. بسته‌های Arch ارائه می‌شوند؛ این راهنما وجود ورودی رسمی AUR را فرض نمی‌کند. تنها روش مناسب توزیع خود را اجرا کنید.

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

## تنظیمات ماندگار

پوشهٔ پیش‌فرض ~/Applications است. می‌توانید آن را در تنظیمات عوض کنید؛ برنامه‌های موجود خودکار منتقل نمی‌شوند. یکی از رابط‌های واقعی GPUI/Ely، Qt 6 Widgets یا GNOME/GTK 4/libadwaita را انتخاب کنید. تغییر رابط ابتدا ترجیحات را ذخیره و سپس مدیر را دوباره اجرا می‌کند. پوسته، پوشه، زبان، برنامهٔ انتخاب‌شده و وضعیت پنجره حفظ می‌شوند. در تنظیمات → زبان یکی از ۲۶ زبان را انتخاب کنید. سطح‌های تشخیص Error، Info و Warning عمداً انگلیسی می‌مانند.

## به‌روزرسانی و حذف

پیش از بررسی به‌روزرسانی، مخزن GitHub برنامه را تأیید کنید. دادهٔ پشتیبانی‌شدهٔ .upd_info خوانده می‌شود یا می‌توانید owner/repository را دستی وارد کنید. انتشار پایدار را بررسی و AppImage متناسب را نصب کنید. اندازه و قالب کنترل می‌شوند؛ اگر چکیده‌ای منتشر شده باشد SHA-256 نیز بررسی می‌شود. جایگزینی اتمی است و نسخهٔ قبلی پشتیبان دارد. شکست به‌روزرسانی برنامهٔ نصب‌شده را حفظ می‌کند.

حذف، برنامه و ورودی‌های دسکتاپ را به زباله‌دان می‌فرستد تا قابل‌بازیابی باشند. تنظیمات شخصی باقی می‌مانند.


## توسعه

[BUILD-AND-RELEASE](docs/BUILD-AND-RELEASE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [i18n](docs/i18n/COVERAGE.md) · [1.0.1](docs/RELEASE-1.0.1.md) · [THIRD-PARTY-NOTICES](docs/THIRD-PARTY-NOTICES.md)

```sh
bun run scripts/check-i18n.ts
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
cargo build --release --locked
bun run scripts/package.ts --formats deb
```

## مجوز و کد

پروژه از AGPL-3.0-only استفاده می‌کند. استفاده، تغییر و توزیع تابع این مجوز است. توزیع و خدمات شبکه‌ای تغییریافته تعهد ارائهٔ کد متناظر دارند. شرایط کامل را در LICENSE بخوانید.

[کد و انتشارها](https://github.com/xingwangzhe/AppimageMannger)

AGPL-3.0-only © 2026 xingwangzhe · xingwangzhe@outlook.com · [LICENSE](LICENSE)
