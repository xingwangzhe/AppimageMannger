# AppimageMannger : démarrage rapide

AppimageMannger gère les applications AppImage avec une interface Linux native. Sélectionnez une application dans la liste à gauche pour afficher à droite son icône, sa version, son architecture, sa taille et son emplacement. L’importation copie le fichier dans le dossier géré et conserve le téléchargement original.

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

## Licence et sources

Le projet est sous AGPL-3.0-only. Utilisation, modification et redistribution respectent cette licence. La redistribution et les services réseau modifiés impliquent des obligations de fourniture du code source correspondant. Consultez LICENSE.

[Sources et versions](https://github.com/xingwangzhe/AppimageMannger)
