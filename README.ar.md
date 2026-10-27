# مدير AppImage

[![Linux CI](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml/badge.svg)](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml)

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Français](README.fr.md) · [Español](README.es.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [Italiano](README.it.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [Bahasa Indonesia](README.id.md) · [ไทย](README.th.md) · [العربية](README.ar.md) · [हिन्दी](README.hi.md) · [বাংলা](README.bn.md) · [فارسی](README.fa.md) · [עברית](README.he.md) · [Svenska](README.sv.md) · [Čeština](README.cs.md) · [Română](README.ro.md)

مدير AppImage تطبيق أصلي لإدارة تطبيقات AppImage على Linux. اختر تطبيقًا من المكتبة لعرض الأيقونة والإصدار والمعمارية والحجم ومسار التثبيت. ينسخ الاستيراد الملف إلى المجلد المُدار مع إبقاء التنزيل الأصلي.

## ثلاث واجهات أصلية

تُظهر جميع الصور الواجهة الإنجليزية للإصدار 1.0.1. تستخدم كل ترجمات README الصور الثلاث نفسها.

### Ely / GPUI

![Ely / GPUI — English interface](docs/images/ely-gpui-en.png)

### Qt 6 Widgets

![Qt 6 Widgets — English interface](docs/images/qt-en.png)

### GNOME / GTK 4 / libadwaita

![GNOME / GTK 4 — English interface](docs/images/gnome-en.png)

## الميزات

- يستبدل الاستيراد الإصدارات الأقدم مع إبقاء التنزيل الأصلي، ويمكن استعادة التثبيت السابق من سلة المهملات.
- أيقونات التطبيقات وتكامل AppImage مع «فتح باستخدام». يعيد `--repair-registration` إصلاح التسجيل.
- تنزيل Releases مباشرة عبر HTTPS مجهول الهوية مع فحص المعمارية وSHA-256 وعرض التقدم الفعلي والنسبة والحجم المنقول. لا حاجة إلى `gh` أو رمز وصول.
- فواصل قابلة للسحب وتفضيلات محفوظة عبر Ely/GPUI وQt 6 Widgets وGNOME/GTK 4، مع أسماء عرض مترجمة إلى 26 لغة.
- قراءة البيانات الوصفية والأيقونات دون تشغيل AppImage أو وصله كنظام ملفات.

## التثبيت والتشغيل

نزّل من Releases الحزمة المناسبة لمعالجك، ثم اختر إحدى الطرق التالية. للنسخة المحمولة، استبدل AppimageMannger.AppImage باسم الملف الحقيقي. تتوفر حزم Arch، لكن الدليل لا يفترض وجود إدراج رسمي في AUR. لا تحتاج إلى تنفيذ كل أوامر التثبيت؛ اختر صيغة واحدة تناسب توزيعتك.

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

## إعدادات تبقى محفوظة

المجلد الافتراضي هو ~/Applications. يمكنك تغييره في الإعدادات، ولا تُنقل التطبيقات الموجودة تلقائيًا. اختر واجهة فعلية من GPUI/Ely أو Qt 6 Widgets أو GNOME/GTK 4/libadwaita. يحفظ تبديل الواجهة التفضيلات ثم يعيد تشغيل المدير. تُحفظ السمة والمجلد واللغة والتطبيق المحدد وحالة النافذة. من الإعدادات → اللغة اختر إحدى اللغات الست والعشرين. تبقى مستويات التشخيص Error وInfo وWarning بالإنجليزية عمدًا.

## التحديث والإزالة

قبل البحث عن تحديث، تحقق من مستودع GitHub الخاص بالتطبيق. تُقرأ بيانات .upd_info المدعومة، أو يمكنك إدخال owner/repository يدويًا. ابحث عن إصدار مستقر وثبّت AppImage الموافق لمعمارية الجهاز. يُفحص الحجم والتنسيق، ويُتحقق من SHA-256 عندما ينشر المطور بصمة. الاستبدال ذري مع نسخة احتياطية من الإصدار السابق؛ الفشل يحافظ على التطبيق المثبّت.

تنقل الإزالة التطبيق وإدخالات سطح المكتب إلى سلة المهملات لإمكان استعادتها. تظل الإعدادات الشخصية محفوظة.


## التطوير

[BUILD-AND-RELEASE](docs/BUILD-AND-RELEASE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [i18n](docs/i18n/COVERAGE.md) · [1.0.1](docs/RELEASE-1.0.1.md) · [THIRD-PARTY-NOTICES](docs/THIRD-PARTY-NOTICES.md)

```sh
bun run scripts/check-i18n.ts
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
cargo build --release --locked
bun run scripts/package.ts --formats deb
```

## الترخيص والمصدر

يستخدم المشروع AGPL-3.0-only. يسمح بالاستخدام والتعديل والتوزيع وفق الترخيص. يترتب على التوزيع والخدمات الشبكية المعدلة توفير المصدر المقابل بحسب الشروط الكاملة في LICENSE.

[المصدر والإصدارات](https://github.com/xingwangzhe/AppimageMannger)

AGPL-3.0-only © 2026 xingwangzhe · xingwangzhe@outlook.com · [LICENSE](LICENSE)
