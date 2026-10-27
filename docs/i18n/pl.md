# AppimageMannger: szybki start

AppimageMannger to natywny menedżer aplikacji AppImage dla Linuksa. Wybierz aplikację z listy po lewej, aby po prawej zobaczyć ikonę, wersję, architekturę, rozmiar i ścieżkę instalacji. Importowanie kopiuje plik do zarządzanego katalogu i zachowuje pierwotnie pobrany plik.

## Instalacja i uruchomienie

Pobierz z Releases pakiet dla swojego procesora i wybierz jedną z poniższych metod. W wersji przenośnej zastąp AppimageMannger.AppImage rzeczywistą nazwą pliku. Udostępniane są pakiety Arch; instrukcja nie zakłada oficjalnego wpisu w AUR.

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

## Trwałe ustawienia

Domyślny katalog to ~/Applications. Zmienisz go w ustawieniach; istniejące aplikacje nie zostaną automatycznie przeniesione. Dostępne są rzeczywiste interfejsy GPUI/Ely, Qt 6 Widgets oraz GNOME/GTK 4/libadwaita. Zmiana zapisuje preferencje i uruchamia menedżer ponownie. Motyw, katalog, język, wybrana aplikacja i stan okna są zachowywane. Ustawienia → Język pozwalają wybrać jeden z 26 języków. Poziomy diagnostyczne Error, Info i Warning celowo pozostają angielskie.

## Aktualizacje i usuwanie

Przed sprawdzeniem aktualizacji zweryfikuj repozytorium GitHub aplikacji. Obsługiwane metadane .upd_info są odczytywane automatycznie; można też podać owner/repository. Wyszukaj stabilne wydanie i zainstaluj odpowiedni AppImage. Sprawdzane są rozmiar i format, a SHA-256 jest weryfikowane, gdy opublikowano skrót. Wymiana jest atomowa i zachowuje kopię poprzedniej wersji. Nieudana aktualizacja pozostawia zainstalowaną aplikację.

Odinstalowanie przenosi aplikację i wpisy pulpitu do kosza, skąd można je przywrócić. Osobista konfiguracja pozostaje zachowana.

## Licencja i źródła

Obowiązuje AGPL-3.0-only. Możesz używać, modyfikować i rozpowszechniać projekt zgodnie z licencją. Dystrybucja i zmienione usługi sieciowe wymagają udostępnienia odpowiednich źródeł. Pełne warunki zawiera LICENSE.

[Kod i wydania](https://github.com/xingwangzhe/AppimageMannger)
