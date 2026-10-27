# מנהל AppImage

[![Linux CI](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml/badge.svg)](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml)

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Français](README.fr.md) · [Español](README.es.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [Italiano](README.it.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [Bahasa Indonesia](README.id.md) · [ไทย](README.th.md) · [العربية](README.ar.md) · [हिन्दी](README.hi.md) · [বাংলা](README.bn.md) · [فارسی](README.fa.md) · [עברית](README.he.md) · [Svenska](README.sv.md) · [Čeština](README.cs.md) · [Română](README.ro.md)

מנהל AppImage הוא מנהל מקומי ליישומי AppImage ב-Linux. בחרו יישום בספרייה כדי לראות את הסמל, הגרסה, הארכיטקטורה, הגודל ומיקום ההתקנה. הייבוא מעתיק את הקובץ לתיקייה המנוהלת ושומר את ההורדה המקורית.

## שלושה ממשקים מקוריים

כל התמונות מציגות את הממשק האנגלי של גרסה 1.0.1. כל תרגומי README משתמשים באותן שלוש תמונות.

### Ely / GPUI

![Ely / GPUI — English interface](docs/images/ely-gpui-en.png)

### Qt 6 Widgets

![Qt 6 Widgets — English interface](docs/images/qt-en.png)

### GNOME / GTK 4 / libadwaita

![GNOME / GTK 4 — English interface](docs/images/gnome-en.png)

## תכונות

- ייבוא מחליף גרסאות ישנות. קובץ ההורדה המקורי נשמר וניתן לשחזר התקנות קודמות מסל המחזור.
- סמלי יישומים ושילוב AppImage בתפריט „פתיחה באמצעות”. הפקודה `--repair-registration` מתקנת את הרישום.
- הורדה ישירה של Releases באמצעות HTTPS אנונימי, בדיקת ארכיטקטורה ו-SHA-256, התקדמות אמיתית, אחוזים וגודל שהועבר. ללא `gh` או אסימון גישה.
- מחיצות נגררות והעדפות שמורות ב-Ely/GPUI, ב-Qt 6 Widgets וב-GNOME/GTK 4; שמות תצוגה ב-26 שפות.
- קריאת מטא-נתונים וסמלים בלי להפעיל או לעגן את AppImage.

## התקנה ופתיחה

הורידו מ־Releases חבילה המתאימה למעבד ובחרו שיטת התקנה אחת למטה. בגרסה הניידת החליפו את AppimageMannger.AppImage בשם הקובץ שהורד. קיימות חבילות Arch; המדריך אינו מניח שיש רישום רשמי ב־AUR. אין צורך להריץ את כל שיטות ההתקנה.

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

## הגדרות שנשמרות

תיקיית ברירת המחדל היא ~/Applications. אפשר לשנות אותה בהגדרות; יישומים קיימים אינם מועברים אוטומטית. בחרו ממשק אמיתי מסוג GPUI/Ely, ‏Qt 6 Widgets או GNOME/GTK 4/libadwaita. מעבר לממשק אחר שומר את ההעדפות ומפעיל מחדש את המנהל. ערכת העיצוב, התיקייה, השפה, היישום הנבחר ומצב החלון נשמרים. בהגדרות → שפה אפשר לבחור אחת מ־26 שפות. רמות האבחון Error, Info ו־Warning נשארות באנגלית בכוונה.

## עדכון והסרה

לפני בדיקת עדכונים ודאו שמאגר GitHub של היישום נכון. המנהל קורא נתוני .upd_info נתמכים, או מאפשר להזין owner/repository ידנית. חפשו גרסה יציבה והתקינו AppImage מתאים. הגודל והפורמט נבדקים; SHA-256 מאומת כאשר פורסם תקציר. ההחלפה אטומית והגרסה הקודמת מגובה. כשל בעדכון שומר על היישום המותקן.

הסרה מעבירה את היישום ואת רשומות שולחן העבודה לאשפה, שממנה אפשר לשחזר אותם. ההגדרות האישיות נשארות.


## פיתוח

[BUILD-AND-RELEASE](docs/BUILD-AND-RELEASE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [i18n](docs/i18n/COVERAGE.md) · [1.0.2](docs/RELEASE-1.0.2.md) · [THIRD-PARTY-NOTICES](docs/THIRD-PARTY-NOTICES.md)

```sh
bun run scripts/check-i18n.ts
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
cargo build --release --locked
bun run scripts/package.ts --formats deb
```

## רישיון וקוד מקור

הפרויקט משתמש ב־AGPL-3.0-only. שימוש, שינוי והפצה כפופים לרישיון. הפצה ושירותי רשת ששונו מחייבים אספקת קוד מקור מתאים לפי התנאים המלאים בקובץ LICENSE.

[קוד מקור וגרסאות](https://github.com/xingwangzhe/AppimageMannger)

AGPL-3.0-only © 2026 [xingwangzhe](https://github.com/xingwangzhe) · [xingwangzhe@outlook.com](mailto:xingwangzhe@outlook.com) · [xingwangzhe.fun](https://xingwangzhe.fun) · [LICENSE](LICENSE)
