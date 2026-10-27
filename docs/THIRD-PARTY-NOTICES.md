# License and dependency notices

Project-authored AppimageMannger code is licensed **AGPL-3.0-only**, as selected by the project owner. The complete text is in [LICENSE](../LICENSE). Version 1.0.0 is distributed with corresponding project source and build instructions.

Qt does not itself impose AGPL on every application. Qt documents commercial and open-source LGPL/GPL licensing options, with module-specific terms. This project uses dynamically linked Qt Core/Gui/Widgets from the system Qt packages; it does not use Qt modules that are exclusively available under a commercial license. See [Qt licensing](https://doc.qt.io/qt-6/licensing.html) and the exact distribution package copyright files for the version shipped.

The project's AGPL choice does not replace any dependency's license:

| Component | Integration | Upstream license/reference |
|---|---|---|
| Ely GPUI Components / Ely Core | Rust UI components | MIT; retain its copyright/license text |
| GPUI and its Rust dependency graph | Rust native renderer/platform | Individual package licenses recorded in Cargo metadata/lockfile and upstream sources |
| Qt 6 Core/Gui/Widgets and platform plugins | Dynamic native helper dependencies | Qt's LGPL/GPL/commercial options and dependency-specific notices |
| GTK 4, libadwaita, json-glib | Dynamic GNOME helper dependencies | Upstream LGPL-family licenses and individual third-party notices |
| Other bundled system libraries/tools | Native package or AppImage runtime | Exact distribution package copyright/license files |

Native deb/rpm/pacman installations use the distribution's dynamic packages and their existing notices. When libraries/plugins are bundled in an AppImage, retain the exact corresponding notices/license texts and package/source provenance; do not claim this document changes their terms. Build metadata records versions and source references. Libraries remain dynamically replaceable rather than statically merged into the Qt/GTK helper.

Authoritative references: [GNU AGPLv3](https://www.gnu.org/licenses/agpl-3.0.html), [Qt licensing](https://doc.qt.io/qt-6/licensing.html), [GTK licensing](https://www.gtk.org/docs/getting-started/licensing/). Check these and the exact shipped dependency sources when changing the distribution model.
