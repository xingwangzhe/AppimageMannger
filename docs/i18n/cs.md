# AppimageMannger: rychlý začátek

AppimageMannger je nativní správce aplikací AppImage pro Linux. Vyberte aplikaci vlevo a vpravo uvidíte její ikonu, verzi, architekturu, velikost a instalační cestu. Import soubor zkopíruje do spravované složky a původní stažený soubor ponechá. V podrobnostech lze aplikaci spustit nebo opravit položku nabídky.

## Instalace a spuštění

Stáhněte z Releases balíček pro svůj procesor a zvolte jednu z následujících možností. U přenosné verze nahraďte AppimageMannger.AppImage skutečným názvem souboru. K dispozici jsou balíčky Arch; návod nepředpokládá oficiální záznam v AUR.

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

## Trvalé nastavení

Výchozí složkou je ~/Applications. Změníte ji v nastavení; stávající aplikace se automaticky nepřesunou. Vyberte skutečné rozhraní GPUI/Ely, Qt 6 Widgets nebo GNOME/GTK 4/libadwaita. Změna uloží předvolby a restartuje správce. Motiv, složka, jazyk, vybraná aplikace i stav okna zůstanou zachovány. Nastavení → Jazyk nabízí 26 jazyků. Úrovně diagnostiky Error, Info a Warning zůstávají záměrně anglické.

## Aktualizace a odinstalace

Před kontrolou aktualizací ověřte repozitář GitHub dané aplikace. Podporovaná metadata .upd_info se načtou, případně zadejte owner/repository ručně. Vyhledejte stabilní vydání a nainstalujte odpovídající AppImage. Kontroluje se velikost a formát; SHA-256 se ověří, pokud vydavatel zveřejnil otisk. Náhrada je atomická a původní verze se zálohuje. Neúspěšná aktualizace zachová nainstalovanou aplikaci.

Odinstalace přesune aplikaci a její položky plochy do koše, odkud je lze obnovit. Osobní nastavení zůstane zachováno. Změna složky ovlivní další skenování a importy, nikoli umístění starých souborů.

## Licence a zdrojový kód

Projekt používá AGPL-3.0-only. Použití, změny a šíření se řídí touto licencí. Distribuce a upravené síťové služby zahrnují povinnost poskytnout odpovídající zdrojový kód. Úplné podmínky najdete v LICENSE.

[Zdrojový kód a vydání](https://github.com/xingwangzhe/AppimageMannger)
