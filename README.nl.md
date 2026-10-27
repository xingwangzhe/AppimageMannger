# AppImage-beheer

[![Linux CI](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml/badge.svg)](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml)

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Français](README.fr.md) · [Español](README.es.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [Italiano](README.it.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [Bahasa Indonesia](README.id.md) · [ไทย](README.th.md) · [العربية](README.ar.md) · [हिन्दी](README.hi.md) · [বাংলা](README.bn.md) · [فارسی](README.fa.md) · [עברית](README.he.md) · [Svenska](README.sv.md) · [Čeština](README.cs.md) · [Română](README.ro.md)

AppImage-beheer beheert AppImage-toepassingen met een echte Linux-interface. Kies links een toepassing om rechts het pictogram, de versie, architectuur, grootte en installatielocatie te bekijken. Importeren kopieert het bestand naar de beheerde map en laat de oorspronkelijke download staan.

## Drie native interfaces

Alle afbeeldingen tonen de Engelse interface van versie 1.0.1. Elke vertaalde README gebruikt dezelfde drie afbeeldingen.

### Ely / GPUI

![Ely / GPUI — English interface](docs/images/ely-gpui-en.png)

### Qt 6 Widgets

![Qt 6 Widgets — English interface](docs/images/qt-en.png)

### GNOME / GTK 4 / libadwaita

![GNOME / GTK 4 — English interface](docs/images/gnome-en.png)

## Functies

- Importeren vervangt oudere versies. De oorspronkelijke download blijft behouden; eerdere installaties zijn vanuit de prullenbak te herstellen.
- Toepassingspictogrammen en AppImage-integratie in ‘Openen met’. `--repair-registration` herstelt de registratie.
- Releases rechtstreeks via anonieme HTTPS downloaden, met controle van architectuur en SHA-256, echte voortgang, percentage en overgedragen omvang. Geen `gh` of token nodig.
- Versleepbare scheidingen en opgeslagen voorkeuren in Ely/GPUI, Qt 6 Widgets en GNOME/GTK 4; weergavenamen in 26 talen.
- Metadata en pictogrammen lezen zonder het AppImage uit te voeren of te koppelen.

## Installeren en openen

Download bij Releases een pakket voor uw processor en kies hieronder één installatiemethode. Vervang voor de draagbare versie AppimageMannger.AppImage door de echte bestandsnaam. Arch-pakketten zijn beschikbaar; deze handleiding gaat niet uit van een officiële AUR-vermelding.

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

## Instellingen die bewaard blijven

De standaardmap is ~/Applications. U kunt deze wijzigen in Instellingen; bestaande toepassingen worden niet automatisch verplaatst. Kies de echte GPUI/Ely-, Qt 6 Widgets- of GNOME/GTK 4/libadwaita-interface. Wisselen slaat voorkeuren op en start de beheerder opnieuw. Thema, map, taal, geselecteerde toepassing en vensterstatus blijven bewaard. Onder Instellingen → Taal kunt u uit 26 talen kiezen. De diagnoseniveaus Error, Info en Warning blijven bewust Engels.

## Bijwerken en verwijderen

Controleer vóór het zoeken naar updates de GitHub-repository van de toepassing. Ondersteunde .upd_info-gegevens worden gelezen; u kunt ook owner/repository invoeren. Zoek een stabiele uitgave en installeer het passende AppImage. Grootte en formaat worden gecontroleerd; SHA-256 wordt geverifieerd wanneer een digest is gepubliceerd. Vervanging gebeurt atomair en bewaart een reservekopie van de vorige versie. Een mislukte update laat de geïnstalleerde toepassing intact.

Verwijderen verplaatst de toepassing en bureaubladvermeldingen naar de prullenbak. Ze kunnen worden hersteld; persoonlijke instellingen blijven behouden.


## Ontwikkeling

[BUILD-AND-RELEASE](docs/BUILD-AND-RELEASE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [i18n](docs/i18n/COVERAGE.md) · [1.0.2](docs/RELEASE-1.0.2.md) · [THIRD-PARTY-NOTICES](docs/THIRD-PARTY-NOTICES.md)

```sh
bun run scripts/check-i18n.ts
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
cargo build --release --locked
bun run scripts/package.ts --formats deb
```

## Licentie en broncode

Het project gebruikt AGPL-3.0-only. Gebruik, wijziging en verspreiding zijn toegestaan volgens die licentie. Verspreiding en gewijzigde netwerkdiensten brengen verplichtingen voor de bijbehorende broncode mee. Lees LICENSE voor alle voorwaarden.

[Broncode en uitgaven](https://github.com/xingwangzhe/AppimageMannger)

AGPL-3.0-only © 2026 [xingwangzhe](https://github.com/xingwangzhe) · [xingwangzhe@outlook.com](mailto:xingwangzhe@outlook.com) · [xingwangzhe.fun](https://xingwangzhe.fun) · [LICENSE](LICENSE)
