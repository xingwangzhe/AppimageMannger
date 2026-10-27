# AppImage-hanterare

[![Linux CI](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml/badge.svg)](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml)

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Français](README.fr.md) · [Español](README.es.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [Italiano](README.it.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [Bahasa Indonesia](README.id.md) · [ไทย](README.th.md) · [العربية](README.ar.md) · [हिन्दी](README.hi.md) · [বাংলা](README.bn.md) · [فارسی](README.fa.md) · [עברית](README.he.md) · [Svenska](README.sv.md) · [Čeština](README.cs.md) · [Română](README.ro.md)

AppImage-hanterare hanterar AppImage-program med ett äkta Linux-gränssnitt. Välj ett program i listan till vänster för att se ikon, version, arkitektur, storlek och installationssökväg till höger. Import kopierar filen till den hanterade mappen och behåller den ursprungliga hämtningen.

## Tre inbyggda gränssnitt

Alla bilder visar det engelska gränssnittet i version 1.0.1. Varje översatt README använder samma tre bilder.

### Ely / GPUI

![Ely / GPUI — English interface](docs/images/ely-gpui-en.png)

### Qt 6 Widgets

![Qt 6 Widgets — English interface](docs/images/qt-en.png)

### GNOME / GTK 4 / libadwaita

![GNOME / GTK 4 — English interface](docs/images/gnome-en.png)

## Funktioner

- Import ersätter äldre versioner. Originalhämtningen behålls och tidigare installationer kan återställas från papperskorgen.
- Programikoner och AppImage-integrering för ”Öppna med”. `--repair-registration` reparerar registreringen.
- Direkta anonyma HTTPS-hämtningar av Releases med arkitektur- och SHA-256-kontroll, verkligt förlopp, procent och överförd storlek. Inget `gh` eller token krävs.
- Dragbara avdelare och sparade inställningar i Ely/GPUI, Qt 6 Widgets och GNOME/GTK 4; visningsnamn på 26 språk.
- Läser metadata och ikoner utan att köra eller montera AppImage.

## Installera och öppna

Hämta ett paket för din processor från Releases och välj en metod nedan. För den portabla versionen ersätter du AppimageMannger.AppImage med det verkliga filnamnet. Arch-paket tillhandahålls; guiden förutsätter ingen officiell AUR-post.

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

## Inställningar som sparas

Standardmappen är ~/Applications. Ändra den i Inställningar; befintliga program flyttas inte automatiskt. Välj ett verkligt gränssnitt: GPUI/Ely, Qt 6 Widgets eller GNOME/GTK 4/libadwaita. Byte sparar inställningarna och startar om hanteraren. Tema, mapp, språk, valt program och fönsterläge bevaras. Under Inställningar → Språk finns 26 språk. Diagnostiknivåerna Error, Info och Warning förblir avsiktligt engelska.

## Uppdatera och avinstallera

Kontrollera programmets GitHub-förråd innan du söker efter uppdateringar. Stödd .upd_info läses automatiskt; du kan också ange owner/repository manuellt. Sök efter en stabil utgåva och installera rätt AppImage. Storlek och format kontrolleras; SHA-256 verifieras när en kontrollsumma har publicerats. Ersättningen är atomär och föregående version säkerhetskopieras. En misslyckad uppdatering bevarar det installerade programmet.

Avinstallation flyttar programmet och dess skrivbordsposter till papperskorgen. Du kan återställa dem, medan personliga inställningar behålls. Ett mappbyte påverkar sökning och kommande importer men flyttar inte gamla filer.


## Utveckling

[BUILD-AND-RELEASE](docs/BUILD-AND-RELEASE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [i18n](docs/i18n/COVERAGE.md) · [1.0.1](docs/RELEASE-1.0.1.md) · [THIRD-PARTY-NOTICES](docs/THIRD-PARTY-NOTICES.md)

```sh
bun run scripts/check-i18n.ts
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
cargo build --release --locked
bun run scripts/package.ts --formats deb
```

## Licens och källkod

Projektet använder AGPL-3.0-only. Användning, ändring och spridning följer licensen. Distribution och ändrade nätverkstjänster medför skyldigheter att tillhandahålla motsvarande källkod. Fullständiga villkor finns i LICENSE.

[Källkod och utgåvor](https://github.com/xingwangzhe/AppimageMannger)

AGPL-3.0-only © 2026 xingwangzhe · xingwangzhe@outlook.com · [LICENSE](LICENSE)
