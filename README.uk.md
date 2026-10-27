# Менеджер AppImage

[![Linux CI](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml/badge.svg)](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml)

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Français](README.fr.md) · [Español](README.es.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [Italiano](README.it.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [Bahasa Indonesia](README.id.md) · [ไทย](README.th.md) · [العربية](README.ar.md) · [हिन्दी](README.hi.md) · [বাংলা](README.bn.md) · [فارسی](README.fa.md) · [עברית](README.he.md) · [Svenska](README.sv.md) · [Čeština](README.cs.md) · [Română](README.ro.md)

Менеджер AppImage — нативний менеджер застосунків AppImage для Linux. Виберіть застосунок у списку ліворуч, щоб праворуч побачити піктограму, версію, архітектуру, розмір і шлях встановлення. Імпорт копіює файл до керованої теки та не видаляє початкове завантаження. У деталях доступні запуск і виправлення пункту меню.

## Три нативні інтерфейси

Усі знімки показують англійський інтерфейс версії 1.0.1. Кожен переклад README використовує ті самі три зображення.

### Ely / GPUI

![Ely / GPUI — English interface](docs/images/ely-gpui-en.png)

### Qt 6 Widgets

![Qt 6 Widgets — English interface](docs/images/qt-en.png)

### GNOME / GTK 4 / libadwaita

![GNOME / GTK 4 — English interface](docs/images/gnome-en.png)

## Можливості

- Імпорт замінює старі версії. Початковий файл зберігається, а попередні встановлення можна відновити з кошика.
- Піктограми й інтеграція AppImage у «Відкрити за допомогою». `--repair-registration` відновлює реєстрацію.
- Пряме анонімне завантаження Releases через HTTPS, перевірка архітектури й SHA-256, справжній поступ, відсоток та обсяг переданих даних. Без `gh` і токена.
- Рухомі роздільники та збережені налаштування в Ely/GPUI, Qt 6 Widgets і GNOME/GTK 4; назви 26 мовами.
- Читання метаданих і піктограм без запуску або монтування AppImage.

## Встановлення й запуск

Завантажте з Releases пакет для свого процесора та виберіть один зі способів нижче. Для переносної версії замініть AppimageMannger.AppImage справжньою назвою файла. Пакети Arch надаються; цей посібник не передбачає офіційної публікації в AUR.

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

## Збережені налаштування

Типова тека — ~/Applications. Її можна змінити в налаштуваннях; наявні застосунки автоматично не переміщуються. Доступні справжні інтерфейси GPUI/Ely, Qt 6 Widgets і GNOME/GTK 4/libadwaita. Зміна зберігає налаштування та перезапускає менеджер. Тема, тека, мова, вибраний застосунок і стан вікна зберігаються. У «Налаштування → Мова» доступні 26 мов. Діагностичні рівні Error, Info та Warning навмисно залишаються англійськими.

## Оновлення й видалення

Перед перевіркою оновлень переконайтеся, що репозиторій GitHub застосунку правильний. Підтримувані метадані .upd_info читаються автоматично; також можна ввести owner/repository вручну. Знайдіть стабільний випуск і встановіть відповідний AppImage. Розмір і формат перевіряються; SHA-256 звіряється, якщо опубліковано хеш. Заміна атомарна, а попередня версія має резервну копію. Невдале оновлення зберігає встановлений застосунок.

Видалення переміщує застосунок і ярлики до кошика. Їх можна відновити; особисті налаштування залишаються.


## Розробка

[BUILD-AND-RELEASE](docs/BUILD-AND-RELEASE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [i18n](docs/i18n/COVERAGE.md) · [1.0.2](docs/RELEASE-1.0.2.md) · [THIRD-PARTY-NOTICES](docs/THIRD-PARTY-NOTICES.md)

```sh
bun run scripts/check-i18n.ts
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
cargo build --release --locked
bun run scripts/package.ts --formats deb
```

## Ліцензія та код

Проєкт використовує AGPL-3.0-only. Використання, зміна та поширення дозволені на її умовах. Поширення й змінені мережеві служби передбачають надання відповідного початкового коду. Повні умови наведено у LICENSE.

[Код і випуски](https://github.com/xingwangzhe/AppimageMannger)

AGPL-3.0-only © 2026 [xingwangzhe](https://github.com/xingwangzhe) · [xingwangzhe@outlook.com](mailto:xingwangzhe@outlook.com) · [xingwangzhe.fun](https://xingwangzhe.fun) · [LICENSE](LICENSE)
