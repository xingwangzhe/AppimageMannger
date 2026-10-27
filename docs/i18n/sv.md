# AppimageMannger: kom igång

AppimageMannger hanterar AppImage-program med ett äkta Linux-gränssnitt. Välj ett program i listan till vänster för att se ikon, version, arkitektur, storlek och installationssökväg till höger. Import kopierar filen till den hanterade mappen och behåller den ursprungliga hämtningen.

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

## Licens och källkod

Projektet använder AGPL-3.0-only. Användning, ändring och spridning följer licensen. Distribution och ändrade nätverkstjänster medför skyldigheter att tillhandahålla motsvarande källkod. Fullständiga villkor finns i LICENSE.

[Källkod och utgåvor](https://github.com/xingwangzhe/AppimageMannger)
