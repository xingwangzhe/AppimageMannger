# Správce AppImage

[![Linux CI](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml/badge.svg)](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml)

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Français](README.fr.md) · [Español](README.es.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [Italiano](README.it.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [Bahasa Indonesia](README.id.md) · [ไทย](README.th.md) · [العربية](README.ar.md) · [हिन्दी](README.hi.md) · [বাংলা](README.bn.md) · [فارسی](README.fa.md) · [עברית](README.he.md) · [Svenska](README.sv.md) · [Čeština](README.cs.md) · [Română](README.ro.md)

Správce AppImage je nativní správce aplikací AppImage pro Linux. Vyberte aplikaci vlevo a vpravo uvidíte její ikonu, verzi, architekturu, velikost a instalační cestu. Import soubor zkopíruje do spravované složky a původní stažený soubor ponechá. V podrobnostech lze aplikaci spustit nebo opravit položku nabídky.

## Tři nativní rozhraní

Všechny obrázky ukazují anglické rozhraní verze 1.0.1. Každý překlad README používá stejné tři snímky.

### Ely / GPUI

![Ely / GPUI — English interface](docs/images/ely-gpui-en.png)

### Qt 6 Widgets

![Qt 6 Widgets — English interface](docs/images/qt-en.png)

### GNOME / GTK 4 / libadwaita

![GNOME / GTK 4 — English interface](docs/images/gnome-en.png)

## Funkce

- Import nahrazuje starší verze. Původní stažený soubor zůstává zachován a předchozí instalace lze obnovit z koše.
- Ikony aplikací a integrace AppImage do nabídky „Otevřít pomocí“. `--repair-registration` opraví registraci.
- Přímé anonymní stahování Releases přes HTTPS s kontrolou architektury a SHA-256, skutečným průběhem, procenty a přenesenou velikostí. Bez `gh` a tokenu.
- Přetahovatelné oddělovače a uložená nastavení v Ely/GPUI, Qt 6 Widgets a GNOME/GTK 4; názvy ve 26 jazycích.
- Čtení metadat a ikon bez spuštění nebo připojení AppImage.

## Instalace a spuštění

Stáhněte z Releases balíček pro svůj procesor a zvolte jednu z následujících možností. U přenosné verze nahraďte AppimageMannger.AppImage skutečným názvem souboru. K dispozici jsou balíčky Arch; návod nepředpokládá oficiální záznam v AUR.

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

## Trvalé nastavení

Výchozí složkou je ~/Applications. Změníte ji v nastavení; stávající aplikace se automaticky nepřesunou. Vyberte skutečné rozhraní GPUI/Ely, Qt 6 Widgets nebo GNOME/GTK 4/libadwaita. Změna uloží předvolby a restartuje správce. Motiv, složka, jazyk, vybraná aplikace i stav okna zůstanou zachovány. Nastavení → Jazyk nabízí 26 jazyků. Úrovně diagnostiky Error, Info a Warning zůstávají záměrně anglické.

## Aktualizace a odinstalace

Před kontrolou aktualizací ověřte repozitář GitHub dané aplikace. Podporovaná metadata .upd_info se načtou, případně zadejte owner/repository ručně. Vyhledejte stabilní vydání a nainstalujte odpovídající AppImage. Kontroluje se velikost a formát; SHA-256 se ověří, pokud vydavatel zveřejnil otisk. Náhrada je atomická a původní verze se zálohuje. Neúspěšná aktualizace zachová nainstalovanou aplikaci.

Odinstalace přesune aplikaci a její položky plochy do koše, odkud je lze obnovit. Osobní nastavení zůstane zachováno. Změna složky ovlivní další skenování a importy, nikoli umístění starých souborů.


## Vývoj

[BUILD-AND-RELEASE](docs/BUILD-AND-RELEASE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [i18n](docs/i18n/COVERAGE.md) · [1.0.3](docs/RELEASE-1.0.3.md) · [THIRD-PARTY-NOTICES](docs/THIRD-PARTY-NOTICES.md)

```sh
bun run scripts/check-i18n.ts
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
cargo build --release --locked
bun run scripts/package.ts --formats deb
```

## Licence a zdrojový kód

Projekt používá AGPL-3.0-only. Použití, změny a šíření se řídí touto licencí. Distribuce a upravené síťové služby zahrnují povinnost poskytnout odpovídající zdrojový kód. Úplné podmínky najdete v LICENSE.

[Zdrojový kód a vydání](https://github.com/xingwangzhe/AppimageMannger)

AGPL-3.0-only © 2026 [xingwangzhe](https://github.com/xingwangzhe) · [xingwangzhe@outlook.com](mailto:xingwangzhe@outlook.com) · [xingwangzhe.fun](https://xingwangzhe.fun) · [LICENSE](LICENSE)
