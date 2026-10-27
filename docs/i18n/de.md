# AppimageMannger: Schnellstart

AppimageMannger verwaltet AppImage-Anwendungen nativ unter Linux. Wählen Sie links eine Anwendung aus; rechts erscheinen Symbol, Version, Architektur, Größe und Installationspfad. Beim Importieren wird die Datei in den verwalteten Ordner kopiert. Die ursprüngliche Download-Datei bleibt erhalten.

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

## Lizenz und Quellcode

Es gilt AGPL-3.0-only. Nutzung, Änderung und Weitergabe sind nach dieser Lizenz erlaubt. Bei Weitergabe und geänderten Netzwerkdiensten gelten Pflichten zur Bereitstellung des zugehörigen Quellcodes. Die vollständigen Bedingungen stehen in LICENSE.

[Quellcode und Veröffentlichungen](https://github.com/xingwangzhe/AppimageMannger)
