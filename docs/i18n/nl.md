# AppimageMannger: snel aan de slag

AppimageMannger beheert AppImage-toepassingen met een echte Linux-interface. Kies links een toepassing om rechts het pictogram, de versie, architectuur, grootte en installatielocatie te bekijken. Importeren kopieert het bestand naar de beheerde map en laat de oorspronkelijke download staan.

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

## Licentie en broncode

Het project gebruikt AGPL-3.0-only. Gebruik, wijziging en verspreiding zijn toegestaan volgens die licentie. Verspreiding en gewijzigde netwerkdiensten brengen verplichtingen voor de bijbehorende broncode mee. Lees LICENSE voor alle voorwaarden.

[Broncode en uitgaven](https://github.com/xingwangzhe/AppimageMannger)
