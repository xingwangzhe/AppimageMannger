# Менеджер AppImage

[![Linux CI](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml/badge.svg)](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml)

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Français](README.fr.md) · [Español](README.es.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [Italiano](README.it.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [Bahasa Indonesia](README.id.md) · [ไทย](README.th.md) · [العربية](README.ar.md) · [हिन्दी](README.hi.md) · [বাংলা](README.bn.md) · [فارسی](README.fa.md) · [עברית](README.he.md) · [Svenska](README.sv.md) · [Čeština](README.cs.md) · [Română](README.ro.md)

Менеджер AppImage — нативный менеджер приложений AppImage для Linux. Выберите приложение в списке слева: справа появятся значок, версия, архитектура, размер и путь установки. Импорт копирует файл в управляемую папку, не удаляя исходную загрузку. В карточке доступны запуск и восстановление пункта меню рабочего стола.

## Три нативных интерфейса

Все снимки показывают английский интерфейс версии 1.0.1. Во всех переводах README используются те же три изображения.

### Ely / GPUI

![Ely / GPUI — English interface](docs/images/ely-gpui-en.png)

### Qt 6 Widgets

![Qt 6 Widgets — English interface](docs/images/qt-en.png)

### GNOME / GTK 4 / libadwaita

![GNOME / GTK 4 — English interface](docs/images/gnome-en.png)

## Возможности

- Импорт заменяет старые версии. Исходный файл сохраняется, а предыдущие установки можно восстановить из корзины.
- Значки приложений и интеграция AppImage в меню «Открыть с помощью». Команда `--repair-registration` восстанавливает регистрацию.
- Прямая анонимная загрузка Releases по HTTPS с проверкой архитектуры и SHA-256, реальным прогрессом, процентом и объёмом передачи. `gh` и токен не нужны.
- Перетаскиваемые разделители и сохранение настроек в Ely/GPUI, Qt 6 Widgets и GNOME/GTK 4; названия приложения на 26 языках.
- Чтение метаданных и значков без запуска или монтирования AppImage.

## Установка и запуск

Скачайте из Releases пакет для своего процессора и выберите один способ ниже. Для переносной версии замените AppimageMannger.AppImage настоящим именем файла. Предоставляются пакеты Arch; официальное размещение в AUR здесь не предполагается.

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

## Сохраняемые настройки

Папка по умолчанию — ~/Applications. Её можно изменить в настройках; существующие приложения автоматически не переносятся. Доступны настоящие интерфейсы GPUI/Ely, Qt 6 Widgets и GNOME/GTK 4/libadwaita. Смена интерфейса сохраняет настройки и перезапускает менеджер. Тема, папка, язык, выбранное приложение и состояние окна сохраняются. В разделе «Настройки → Язык» доступны 26 языков. Уровни диагностики Error, Info и Warning намеренно остаются английскими.

## Обновления и удаление

Перед проверкой укажите правильный репозиторий GitHub приложения. Поддерживаемые метаданные .upd_info распознаются автоматически; также можно ввести owner/repository вручную. Найдите стабильный выпуск и установите подходящий AppImage. Проверяются размер и формат; SHA-256 сверяется, если опубликована контрольная сумма. Замена атомарна, предыдущая версия сохраняется в резервной копии. Ошибка обновления не уничтожает установленное приложение.

Удаление перемещает приложение и его ярлыки в корзину. Их можно восстановить, личные настройки остаются на месте.


## Разработка

[BUILD-AND-RELEASE](docs/BUILD-AND-RELEASE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [i18n](docs/i18n/COVERAGE.md) · [1.0.2](docs/RELEASE-1.0.2.md) · [THIRD-PARTY-NOTICES](docs/THIRD-PARTY-NOTICES.md)

```sh
bun run scripts/check-i18n.ts
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
cargo build --release --locked
bun run scripts/package.ts --formats deb
```

## Лицензия и исходники

Проект распространяется по AGPL-3.0-only. Разрешены использование, изменение и распространение на её условиях. Распространение и изменённые сетевые службы предполагают предоставление соответствующих исходников. Полные условия приведены в LICENSE.

[Исходники и выпуски](https://github.com/xingwangzhe/AppimageMannger)

AGPL-3.0-only © 2026 [xingwangzhe](https://github.com/xingwangzhe) · [xingwangzhe@outlook.com](mailto:xingwangzhe@outlook.com) · [xingwangzhe.fun](https://xingwangzhe.fun) · [LICENSE](LICENSE)
