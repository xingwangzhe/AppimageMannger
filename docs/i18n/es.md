# AppimageMannger: inicio rápido

AppimageMannger administra aplicaciones AppImage mediante una interfaz nativa de Linux. Seleccione una aplicación en la lista izquierda para ver a la derecha su icono, versión, arquitectura, tamaño y ubicación. Importar copia el archivo a la carpeta administrada y conserva la descarga original.

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

## Licencia y código

Rige AGPL-3.0-only. Puede usar, modificar y redistribuir el proyecto conforme a ella. La distribución y los servicios de red modificados conllevan obligaciones de entregar el código correspondiente. Consulte LICENSE.

[Código y versiones](https://github.com/xingwangzhe/AppimageMannger)
