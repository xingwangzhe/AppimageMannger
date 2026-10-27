# Manager AppImage

[![Linux CI](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml/badge.svg)](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml)

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Français](README.fr.md) · [Español](README.es.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [Italiano](README.it.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [Bahasa Indonesia](README.id.md) · [ไทย](README.th.md) · [العربية](README.ar.md) · [हिन्दी](README.hi.md) · [বাংলা](README.bn.md) · [فارسی](README.fa.md) · [עברית](README.he.md) · [Svenska](README.sv.md) · [Čeština](README.cs.md) · [Română](README.ro.md)

Manager AppImage este un gestionar nativ de aplicații AppImage pentru Linux. Selectați o aplicație din lista stângă pentru a vedea în dreapta pictograma, versiunea, arhitectura, dimensiunea și calea de instalare. Importul copiază fișierul în dosarul gestionat și păstrează descărcarea originală.

## Trei interfețe native

Toate imaginile arată interfața în engleză a versiunii 1.0.1. Fiecare README tradus folosește aceleași trei capturi.

### Ely / GPUI

![Ely / GPUI — English interface](docs/images/ely-gpui-en.png)

### Qt 6 Widgets

![Qt 6 Widgets — English interface](docs/images/qt-en.png)

### GNOME / GTK 4 / libadwaita

![GNOME / GTK 4 — English interface](docs/images/gnome-en.png)

## Funcționalități

- Importul înlocuiește versiunile vechi. Descărcarea originală rămâne, iar instalările anterioare pot fi restaurate din coș.
- Pictograme și integrarea AppImage în „Deschide cu”. `--repair-registration` repară înregistrarea.
- Descărcări directe de Releases prin HTTPS anonim, cu verificarea arhitecturii și SHA-256, progres real, procent și volum transferat. Fără `gh` sau token.
- Separatoare glisabile și preferințe salvate în Ely/GPUI, Qt 6 Widgets și GNOME/GTK 4; nume afișate în 26 de limbi.
- Citește metadatele și pictogramele fără a executa sau monta AppImage.

## Instalare și deschidere

Descărcați din Releases pachetul potrivit procesorului și alegeți o metodă de mai jos. Pentru versiunea portabilă, înlocuiți AppimageMannger.AppImage cu numele real al fișierului. Sunt oferite pachete Arch; ghidul nu presupune o înregistrare oficială în AUR.

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

## Preferințe păstrate

Dosarul implicit este ~/Applications. Îl puteți schimba în Setări; aplicațiile existente nu sunt mutate automat. Alegeți interfața reală GPUI/Ely, Qt 6 Widgets sau GNOME/GTK 4/libadwaita. Schimbarea salvează preferințele și repornește gestionarul. Tema, dosarul, limba, aplicația selectată și starea ferestrei sunt păstrate. Setări → Limbă oferă 26 de limbi. Nivelurile de diagnostic Error, Info și Warning rămân intenționat în engleză.

## Actualizare și dezinstalare

Înainte de verificare, confirmați depozitul GitHub al aplicației. Sunt citite metadatele .upd_info acceptate sau puteți introduce owner/repository manual. Căutați o versiune stabilă și instalați AppImage corespunzător. Se verifică dimensiunea și formatul; SHA-256 este validat când se publică un rezumat. Înlocuirea este atomică și versiunea veche este salvată. O actualizare eșuată păstrează aplicația instalată.

Dezinstalarea mută aplicația și intrările desktop în Coș, de unde pot fi restaurate. Configurația personală rămâne disponibilă. Schimbarea dosarului afectează scanarea și importurile viitoare, fără să mute fișierele vechi.


## Dezvoltare

[BUILD-AND-RELEASE](docs/BUILD-AND-RELEASE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [i18n](docs/i18n/COVERAGE.md) · [1.0.3](docs/RELEASE-1.0.3.md) · [THIRD-PARTY-NOTICES](docs/THIRD-PARTY-NOTICES.md)

```sh
bun run scripts/check-i18n.ts
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
cargo build --release --locked
bun run scripts/package.ts --formats deb
```

## Licență și surse

Proiectul folosește AGPL-3.0-only. Utilizarea, modificarea și redistribuirea respectă această licență. Distribuția și serviciile de rețea modificate implică furnizarea surselor corespunzătoare. Consultați LICENSE pentru termenii compleți.

[Surse și versiuni](https://github.com/xingwangzhe/AppimageMannger)

AGPL-3.0-only © 2026 [xingwangzhe](https://github.com/xingwangzhe) · [xingwangzhe@outlook.com](mailto:xingwangzhe@outlook.com) · [xingwangzhe.fun](https://xingwangzhe.fun) · [LICENSE](LICENSE)
