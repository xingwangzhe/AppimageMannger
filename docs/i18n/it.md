# AppimageMannger: guida rapida

AppimageMannger gestisce le applicazioni AppImage con un’interfaccia Linux nativa. Seleziona un’app nell’elenco a sinistra per vedere a destra icona, versione, architettura, dimensioni e percorso. L’importazione copia il file nella cartella gestita e mantiene il download originale.

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

## Licenza e sorgenti

Il progetto usa AGPL-3.0-only. Uso, modifica e redistribuzione seguono questa licenza. Distribuzione e servizi di rete modificati comportano obblighi sul codice sorgente corrispondente. Consulta LICENSE per le condizioni complete.

[Sorgenti e versioni](https://github.com/xingwangzhe/AppimageMannger)
