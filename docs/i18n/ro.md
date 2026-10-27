# AppimageMannger: pornire rapidă

AppimageMannger este un gestionar nativ de aplicații AppImage pentru Linux. Selectați o aplicație din lista stângă pentru a vedea în dreapta pictograma, versiunea, arhitectura, dimensiunea și calea de instalare. Importul copiază fișierul în dosarul gestionat și păstrează descărcarea originală.

## Instalare și deschidere

Descărcați din Releases pachetul potrivit procesorului și alegeți o metodă de mai jos. Pentru versiunea portabilă, înlocuiți AppimageMannger.AppImage cu numele real al fișierului. Sunt oferite pachete Arch; ghidul nu presupune o înregistrare oficială în AUR.

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

## Preferințe păstrate

Dosarul implicit este ~/Applications. Îl puteți schimba în Setări; aplicațiile existente nu sunt mutate automat. Alegeți interfața reală GPUI/Ely, Qt 6 Widgets sau GNOME/GTK 4/libadwaita. Schimbarea salvează preferințele și repornește gestionarul. Tema, dosarul, limba, aplicația selectată și starea ferestrei sunt păstrate. Setări → Limbă oferă 26 de limbi. Nivelurile de diagnostic Error, Info și Warning rămân intenționat în engleză.

## Actualizare și dezinstalare

Înainte de verificare, confirmați depozitul GitHub al aplicației. Sunt citite metadatele .upd_info acceptate sau puteți introduce owner/repository manual. Căutați o versiune stabilă și instalați AppImage corespunzător. Se verifică dimensiunea și formatul; SHA-256 este validat când se publică un rezumat. Înlocuirea este atomică și versiunea veche este salvată. O actualizare eșuată păstrează aplicația instalată.

Dezinstalarea mută aplicația și intrările desktop în Coș, de unde pot fi restaurate. Configurația personală rămâne disponibilă. Schimbarea dosarului afectează scanarea și importurile viitoare, fără să mute fișierele vechi.

## Licență și surse

Proiectul folosește AGPL-3.0-only. Utilizarea, modificarea și redistribuirea respectă această licență. Distribuția și serviciile de rețea modificate implică furnizarea surselor corespunzătoare. Consultați LICENSE pentru termenii compleți.

[Surse și versiuni](https://github.com/xingwangzhe/AppimageMannger)
