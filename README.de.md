# AppImage-Verwaltung

[![Linux CI](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml/badge.svg)](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml)

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Français](README.fr.md) · [Español](README.es.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [Italiano](README.it.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [Bahasa Indonesia](README.id.md) · [ไทย](README.th.md) · [العربية](README.ar.md) · [हिन्दी](README.hi.md) · [বাংলা](README.bn.md) · [فارسی](README.fa.md) · [עברית](README.he.md) · [Svenska](README.sv.md) · [Čeština](README.cs.md) · [Română](README.ro.md)

AppImage-Verwaltung verwaltet AppImage-Anwendungen nativ unter Linux. Wählen Sie links eine Anwendung aus; rechts erscheinen Symbol, Version, Architektur, Größe und Installationspfad. Beim Importieren wird die Datei in den verwalteten Ordner kopiert. Die ursprüngliche Download-Datei bleibt erhalten.

## Drei native Oberflächen

Alle Bilder zeigen die englische Oberfläche von Version 1.0.1. Jede übersetzte README verwendet dieselben drei Bilder.

### Ely / GPUI

![Ely / GPUI — English interface](docs/images/ely-gpui-en.png)

### Qt 6 Widgets

![Qt 6 Widgets — English interface](docs/images/qt-en.png)

### GNOME / GTK 4 / libadwaita

![GNOME / GTK 4 — English interface](docs/images/gnome-en.png)

## Funktionen

- Der Import ersetzt ältere Versionen. Die ursprüngliche Datei bleibt erhalten; vorherige Installationen lassen sich aus dem Papierkorb wiederherstellen.
- Anwendungssymbole und AppImage-Integration für „Öffnen mit“. `--repair-registration` repariert die Registrierung.
- Anonyme HTTPS-Downloads von Releases mit Architektur- und SHA-256-Prüfung sowie echtem Fortschritt, Prozentwert und Datenmenge. Kein `gh` oder Token erforderlich.
- Verschiebbare Trennlinien und gespeicherte Einstellungen in Ely/GPUI, Qt 6 Widgets und GNOME/GTK 4; Anzeigenamen in 26 Sprachen.
- Metadaten und Symbole werden gelesen, ohne das AppImage auszuführen oder einzuhängen.

## Installieren und starten

Laden Sie unter Releases ein Paket für Ihren Prozessor herunter und wählen Sie einen der folgenden Wege. Ersetzen Sie bei der portablen Variante AppimageMannger.AppImage durch den tatsächlichen Dateinamen. Arch-Pakete werden angeboten; eine offizielle AUR-Veröffentlichung wird hier nicht vorausgesetzt.

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

## Dauerhafte Einstellungen

Der Standardordner ist ~/Applications. Sie können ihn in den Einstellungen ändern; vorhandene Anwendungen werden dabei nicht verschoben. Zur Auswahl stehen die tatsächlichen Oberflächen GPUI/Ely, Qt 6 Widgets und GNOME/GTK 4/libadwaita. Ein Wechsel speichert die Einstellungen und startet den Manager neu. Design, Ordner, Sprache, ausgewählte Anwendung und Fensterzustand bleiben erhalten. Unter Einstellungen → Sprache stehen 26 Sprachen bereit. Die Diagnoseebenen Error, Info und Warning bleiben absichtlich Englisch.

## Aktualisieren und entfernen

Prüfen Sie vor der Aktualisierung das GitHub-Repository der Anwendung. Unterstützte .upd_info-Metadaten werden erkannt; alternativ tragen Sie owner/repository ein. Suchen Sie eine stabile Veröffentlichung und installieren Sie das passende AppImage. Größe und Format werden geprüft; ein veröffentlichter SHA-256-Digest wird ebenfalls verifiziert. Die Ersetzung erfolgt atomar mit Sicherung der Vorgängerversion. Bei einem Fehler bleibt die installierte Anwendung erhalten.

Deinstallieren verschiebt Anwendung und Desktop-Einträge in den Papierkorb. Sie können diese wiederherstellen; persönliche Einstellungen bleiben bestehen.


## Entwicklung

[BUILD-AND-RELEASE](docs/BUILD-AND-RELEASE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [i18n](docs/i18n/COVERAGE.md) · [1.0.3](docs/RELEASE-1.0.3.md) · [THIRD-PARTY-NOTICES](docs/THIRD-PARTY-NOTICES.md)

```sh
bun run scripts/check-i18n.ts
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
cargo build --release --locked
bun run scripts/package.ts --formats deb
```

## Lizenz und Quellcode

Es gilt AGPL-3.0-only. Nutzung, Änderung und Weitergabe sind nach dieser Lizenz erlaubt. Bei Weitergabe und geänderten Netzwerkdiensten gelten Pflichten zur Bereitstellung des zugehörigen Quellcodes. Die vollständigen Bedingungen stehen in LICENSE.

[Quellcode und Veröffentlichungen](https://github.com/xingwangzhe/AppimageMannger)

AGPL-3.0-only © 2026 [xingwangzhe](https://github.com/xingwangzhe) · [xingwangzhe@outlook.com](mailto:xingwangzhe@outlook.com) · [xingwangzhe.fun](https://xingwangzhe.fun) · [LICENSE](LICENSE)
