# Gestor de AppImage

[![Linux CI](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml/badge.svg)](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml)

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Français](README.fr.md) · [Español](README.es.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [Italiano](README.it.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [Bahasa Indonesia](README.id.md) · [ไทย](README.th.md) · [العربية](README.ar.md) · [हिन्दी](README.hi.md) · [বাংলা](README.bn.md) · [فارسی](README.fa.md) · [עברית](README.he.md) · [Svenska](README.sv.md) · [Čeština](README.cs.md) · [Română](README.ro.md)

Gestor de AppImage administra aplicaciones AppImage mediante una interfaz nativa de Linux. Seleccione una aplicación en la lista izquierda para ver a la derecha su icono, versión, arquitectura, tamaño y ubicación. Importar copia el archivo a la carpeta administrada y conserva la descarga original.

## Tres interfaces nativas

Todas las capturas muestran la interfaz en inglés de la versión 1.0.1. Los README traducidos usan las mismas tres imágenes.

### Ely / GPUI

![Ely / GPUI — English interface](docs/images/ely-gpui-en.png)

### Qt 6 Widgets

![Qt 6 Widgets — English interface](docs/images/qt-en.png)

### GNOME / GTK 4 / libadwaita

![GNOME / GTK 4 — English interface](docs/images/gnome-en.png)

## Funciones

- La importación sustituye las versiones anteriores. Conserva la descarga original y permite recuperar las instalaciones anteriores desde la papelera.
- Iconos e integración de AppImage en «Abrir con». `--repair-registration` repara el registro.
- Descargas directas de Releases por HTTPS anónimo, comprobación de arquitectura y SHA-256, progreso real, porcentaje y tamaño transferido. No requiere `gh` ni token.
- Separadores ajustables y preferencias guardadas en Ely/GPUI, Qt 6 Widgets y GNOME/GTK 4; nombres visibles en 26 idiomas.
- Lee metadatos e iconos sin ejecutar ni montar el AppImage.

## Instalar y abrir

Descargue de Releases el paquete adecuado para su procesador y elija un método de abajo. Para la versión portátil, sustituya AppimageMannger.AppImage por el nombre real del archivo. Se ofrecen paquetes Arch; esta guía no presupone una publicación oficial en AUR.

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

## Ajustes que se conservan

La carpeta predeterminada es ~/Applications. Puede cambiarla en Ajustes; las aplicaciones existentes no se trasladan automáticamente. Elija una interfaz real GPUI/Ely, Qt 6 Widgets o GNOME/GTK 4/libadwaita. El cambio guarda las preferencias y reinicia el administrador. Se conservan tema, carpeta, idioma, aplicación seleccionada y estado de la ventana. Ajustes → Idioma permite elegir entre 26 idiomas. Los niveles de diagnóstico Error, Info y Warning permanecen deliberadamente en inglés.

## Actualizar y desinstalar

Antes de buscar actualizaciones, compruebe el repositorio GitHub de la aplicación. Se leen los metadatos .upd_info compatibles, o puede introducir owner/repository. Busque una versión estable e instale el AppImage correspondiente. Se comprueban tamaño y formato; SHA-256 se verifica cuando se publica un resumen. La sustitución es atómica y guarda la versión anterior. Si falla, se conserva la aplicación instalada.

Desinstalar mueve la aplicación y sus entradas de escritorio a la papelera. Puede restaurarlas y la configuración personal permanece intacta.


## Desarrollo

[BUILD-AND-RELEASE](docs/BUILD-AND-RELEASE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [i18n](docs/i18n/COVERAGE.md) · [1.0.3](docs/RELEASE-1.0.3.md) · [THIRD-PARTY-NOTICES](docs/THIRD-PARTY-NOTICES.md)

```sh
bun run scripts/check-i18n.ts
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
cargo build --release --locked
bun run scripts/package.ts --formats deb
```

## Licencia y código

Rige AGPL-3.0-only. Puede usar, modificar y redistribuir el proyecto conforme a ella. La distribución y los servicios de red modificados conllevan obligaciones de entregar el código correspondiente. Consulte LICENSE.

[Código y versiones](https://github.com/xingwangzhe/AppimageMannger)

AGPL-3.0-only © 2026 [xingwangzhe](https://github.com/xingwangzhe) · [xingwangzhe@outlook.com](mailto:xingwangzhe@outlook.com) · [xingwangzhe.fun](https://xingwangzhe.fun) · [LICENSE](LICENSE)
