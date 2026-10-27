# Gestore AppImage

[![Linux CI](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml/badge.svg)](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml)

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Français](README.fr.md) · [Español](README.es.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [Italiano](README.it.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [Bahasa Indonesia](README.id.md) · [ไทย](README.th.md) · [العربية](README.ar.md) · [हिन्दी](README.hi.md) · [বাংলা](README.bn.md) · [فارسی](README.fa.md) · [עברית](README.he.md) · [Svenska](README.sv.md) · [Čeština](README.cs.md) · [Română](README.ro.md)

Gestore AppImage gestisce le applicazioni AppImage con un’interfaccia Linux nativa. Seleziona un’app nell’elenco a sinistra per vedere a destra icona, versione, architettura, dimensioni e percorso. L’importazione copia il file nella cartella gestita e mantiene il download originale.

## Tre interfacce native

Tutte le immagini mostrano l’interfaccia inglese della versione 1.0.1. Ogni README tradotto usa le stesse tre immagini.

### Ely / GPUI

![Ely / GPUI — English interface](docs/images/ely-gpui-en.png)

### Qt 6 Widgets

![Qt 6 Widgets — English interface](docs/images/qt-en.png)

### GNOME / GTK 4 / libadwaita

![GNOME / GTK 4 — English interface](docs/images/gnome-en.png)

## Funzionalità

- L’importazione sostituisce le versioni precedenti. Il download originale resta disponibile e le vecchie installazioni si possono recuperare dal cestino.
- Icone e integrazione AppImage in «Apri con». `--repair-registration` ripara la registrazione.
- Download diretti delle Release tramite HTTPS anonimo, controlli di architettura e SHA-256, avanzamento reale, percentuale e dati trasferiti. Non servono `gh` né token.
- Divisori trascinabili e preferenze salvate in Ely/GPUI, Qt 6 Widgets e GNOME/GTK 4; nomi visualizzati in 26 lingue.
- Lettura di metadati e icone senza eseguire o montare l’AppImage.

## Installazione e avvio

Scarica da Releases un pacchetto adatto al processore e scegli un metodo qui sotto. Per la versione portatile, sostituisci AppimageMannger.AppImage con il nome effettivo del file. Sono disponibili pacchetti Arch; questa guida non presume una pubblicazione ufficiale su AUR.

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

## Preferenze persistenti

La cartella predefinita è ~/Applications. Puoi cambiarla nelle impostazioni; le applicazioni esistenti non vengono spostate automaticamente. Scegli un vero frontend GPUI/Ely, Qt 6 Widgets o GNOME/GTK 4/libadwaita. Il cambio salva le preferenze e riavvia il gestore. Tema, cartella, lingua, applicazione selezionata e stato della finestra sono conservati. Impostazioni → Lingua offre 26 lingue. I livelli diagnostici Error, Info e Warning restano intenzionalmente in inglese.

## Aggiornamenti e rimozione

Prima di cercare aggiornamenti, verifica il repository GitHub dell’applicazione. I metadati .upd_info supportati vengono riconosciuti; puoi anche inserire owner/repository manualmente. Cerca una versione stabile e installa l’AppImage corrispondente. Dimensioni e formato vengono controllati; SHA-256 è verificato quando viene pubblicata un’impronta. La sostituzione è atomica e conserva una copia della versione precedente. Un errore mantiene intatta l’applicazione installata.

La disinstallazione sposta l’app e le relative voci desktop nel cestino. Puoi ripristinarle e le preferenze personali rimangono disponibili.


## Sviluppo

[BUILD-AND-RELEASE](docs/BUILD-AND-RELEASE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [i18n](docs/i18n/COVERAGE.md) · [1.0.2](docs/RELEASE-1.0.2.md) · [THIRD-PARTY-NOTICES](docs/THIRD-PARTY-NOTICES.md)

```sh
bun run scripts/check-i18n.ts
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
cargo build --release --locked
bun run scripts/package.ts --formats deb
```

## Licenza e sorgenti

Il progetto usa AGPL-3.0-only. Uso, modifica e redistribuzione seguono questa licenza. Distribuzione e servizi di rete modificati comportano obblighi sul codice sorgente corrispondente. Consulta LICENSE per le condizioni complete.

[Sorgenti e versioni](https://github.com/xingwangzhe/AppimageMannger)

AGPL-3.0-only © 2026 [xingwangzhe](https://github.com/xingwangzhe) · [xingwangzhe@outlook.com](mailto:xingwangzhe@outlook.com) · [xingwangzhe.fun](https://xingwangzhe.fun) · [LICENSE](LICENSE)
