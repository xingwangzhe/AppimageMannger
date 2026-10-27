# AppimageMannger: Quick start

AppimageMannger is a native Linux manager for AppImage applications. Select an application in the left list to see its version, architecture, size, icon and installation path on the right. Import copies an application into the managed directory; it does not remove the original download.

## Install and open

Download the package for your processor from Releases. Choose one installation method below. For the portable method, replace AppimageMannger.AppImage with the downloaded filename. Arch packages are provided; this guide does not assume an official AUR listing.

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

## Settings that remain saved

The default directory is ~/Applications. Change it in Settings; existing applications are not moved automatically. Choose the actual GPUI/Ely, Qt 6 Widgets or GNOME/GTK 4/libadwaita frontend. Changing the frontend saves your preferences and restarts the manager. Theme, directory, language, selected application and window state are retained. Open Settings → Language to choose one of 26 languages. Error, Info and Warning diagnostic levels deliberately remain in English.

## Updates and removal

Before checking updates, confirm the application’s GitHub repository. The manager reads supported .upd_info metadata or accepts owner/repository manually. Check for a stable release, then install its matching AppImage. Downloads are size and format checked; SHA-256 is verified when a digest is published. Replacement is atomic and the previous version is backed up. A failed update preserves the installed application.

Uninstall moves the application and its desktop entries to Trash, where they can be restored. Personal configuration remains available.

## License and source

The project uses AGPL-3.0-only. You may use, modify and redistribute it under that license. Distribution and modified network services carry corresponding-source obligations; consult LICENSE for the full terms.

[Source and releases](https://github.com/xingwangzhe/AppimageMannger)
