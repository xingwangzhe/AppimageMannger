# Gestionnaire AppImage

[![Linux CI](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml/badge.svg)](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml)

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Français](README.fr.md) · [Español](README.es.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [Italiano](README.it.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [Bahasa Indonesia](README.id.md) · [ไทย](README.th.md) · [العربية](README.ar.md) · [हिन्दी](README.hi.md) · [বাংলা](README.bn.md) · [فارسی](README.fa.md) · [עברית](README.he.md) · [Svenska](README.sv.md) · [Čeština](README.cs.md) · [Română](README.ro.md)

Gestionnaire AppImage gère les applications AppImage avec une interface Linux native. Sélectionnez une application dans la liste à gauche pour afficher à droite son icône, sa version, son architecture, sa taille et son emplacement. L’importation copie le fichier dans le dossier géré et conserve le téléchargement original.

## Trois interfaces natives

Les captures montrent toutes l’interface anglaise de la version 1.0.1. Chaque README traduit utilise les trois mêmes images.

### Ely / GPUI

![Ely / GPUI — English interface](docs/images/ely-gpui-en.png)

### Qt 6 Widgets

![Qt 6 Widgets — English interface](docs/images/qt-en.png)

### GNOME / GTK 4 / libadwaita

![GNOME / GTK 4 — English interface](docs/images/gnome-en.png)

## Fonctionnalités

- L’import remplace les anciennes versions. Le téléchargement d’origine est conservé et les versions précédentes restent récupérables dans la corbeille.
- Icônes et intégration AppImage dans « Ouvrir avec ». `--repair-registration` répare l’enregistrement.
- Téléchargement direct des Releases par HTTPS anonyme, vérification de l’architecture et de SHA-256, progression réelle, pourcentage et volume transféré. Aucun `gh` ni jeton requis.
- Séparateurs déplaçables et préférences enregistrées avec Ely/GPUI, Qt 6 Widgets et GNOME/GTK 4 ; noms affichés dans 26 langues.
- Lecture des métadonnées et des icônes sans exécuter ni monter l’AppImage.

## Installer et ouvrir

Téléchargez depuis Releases un paquet adapté à votre processeur, puis choisissez une méthode ci-dessous. Pour la version portable, remplacez AppimageMannger.AppImage par le véritable nom du fichier. Des paquets Arch sont fournis ; ce guide ne suppose pas une publication officielle dans AUR.

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

## Des réglages conservés

Le dossier par défaut est ~/Applications. Modifiez-le dans les paramètres ; les applications existantes ne sont pas déplacées automatiquement. Choisissez une véritable interface GPUI/Ely, Qt 6 Widgets ou GNOME/GTK 4/libadwaita. Le changement enregistre les préférences et redémarre le gestionnaire. Le thème, le dossier, la langue, la sélection et l’état de la fenêtre sont conservés. Paramètres → Langue propose 26 langues. Les niveaux de diagnostic Error, Info et Warning restent volontairement en anglais.

## Mettre à jour et désinstaller

Avant toute recherche, vérifiez le dépôt GitHub de l’application. Les métadonnées .upd_info prises en charge sont reconnues, ou vous pouvez saisir owner/repository. Recherchez une version stable puis installez l’AppImage correspondant. La taille et le format sont contrôlés ; SHA-256 est vérifié lorsqu’une empreinte est publiée. Le remplacement est atomique et l’ancienne version est sauvegardée. Un échec préserve l’application installée.

La désinstallation déplace l’application et ses raccourcis dans la corbeille. Ils restent restaurables et les réglages personnels sont conservés.


## Développement

[BUILD-AND-RELEASE](docs/BUILD-AND-RELEASE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [i18n](docs/i18n/COVERAGE.md) · [1.0.2](docs/RELEASE-1.0.2.md) · [THIRD-PARTY-NOTICES](docs/THIRD-PARTY-NOTICES.md)

```sh
bun run scripts/check-i18n.ts
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
cargo build --release --locked
bun run scripts/package.ts --formats deb
```

## Licence et sources

Le projet est sous AGPL-3.0-only. Utilisation, modification et redistribution respectent cette licence. La redistribution et les services réseau modifiés impliquent des obligations de fourniture du code source correspondant. Consultez LICENSE.

[Sources et versions](https://github.com/xingwangzhe/AppimageMannger)

AGPL-3.0-only © 2026 [xingwangzhe](https://github.com/xingwangzhe) · [xingwangzhe@outlook.com](mailto:xingwangzhe@outlook.com) · [xingwangzhe.fun](https://xingwangzhe.fun) · [LICENSE](LICENSE)
