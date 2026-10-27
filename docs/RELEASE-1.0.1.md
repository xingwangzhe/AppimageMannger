# 1.0.1 release record

## Changes

- Imports replace versions of the same application, keep source files and preserve installed paths/settings. Older duplicates and their launchers move to restorable Trash. Library locks serialize mutations; failed import transactions restore binaries and launchers.
- The manager icon is installed as PNG and SVG, embedded for portable launches, set on Qt/GPUI windows and resolved through the GTK application-ID desktop alias.
- Desktop MIME registration advertises AppImage Open With support via --import %F. --repair-registration restores it.
- GPUI/Ely SplitPane, Qt QSplitter and GTK GtkPaned provide draggable dividers with a shared persisted fraction.
- All three frontends show a real download progress bar, percentage and transferred bytes independently of the serialized operation queue. Progress records update atomically during transfer and are removed on success/failure. Slow downloads have a longer bounded timeout.
- Updates use anonymous HTTPS Release pages and direct attachment downloads. API fallback is optional; installed clients require neither gh nor a token. Architecture, publisher SHA-256, format, transfer bounds and atomic rollback checks remain enforced.
- Application display names in windows, About and desktop launchers are localized in all 26 supported languages. Stable executable/package/configuration identifiers remain compatible.

## Verification

- Strict Clippy, formatting and 34 release regression tests passed, including a paused real HTTP transfer whose reported byte count is checked before atomic replacement.
- All 26 catalogs, localized desktop/application names, placeholders and QuickStarts passed validation.
- A real anonymous HTTPS check with an empty PATH and no GH_TOKEN/GITHUB_TOKEN found GpgFrontend 2.2.3, its x86_64 direct release URL and publisher SHA-256. No update was performed on the original application.
- Real Qt German and GPUI Arabic windows rendered localized names and application icons; the GPUI RTL library appeared on the right.
- Built and inspected dist/appimage-mannger_1.0.1_amd64.deb (SHA-256 839f940d0bcdc53a5029c86958d35169aa0c9a0320149898d53a26a20e5afb70), then upgraded the installed package from 1.0.0 with pkexec apt-get. dpkg and the installed CLI confirmed 1.0.1.
- The installed engine replaced two isolated import versions with one managed executable while keeping both sources. GIO listed the manager as registered/recommended for both AppImage MIME types.
- Installed GTK rendered the localized Chinese name with the correct desktop application ID and an actual live progress bar reporting percentage and transferred/total bytes (observed 2.9%, 2.1 MB / 73.1 MB).
- The owner confirmed completing local testing and requested that further local tests stop. No exhaustive 26-language by 3-toolkit visual test matrix is claimed.
- Tagged CI succeeded for native x86_64 and aarch64 plus release assembly: https://github.com/xingwangzhe/AppimageMannger/actions/runs/40462133417 .
- Published stable v1.0.1 with 16 assets: https://github.com/xingwangzhe/AppimageMannger/releases/tag/v1.0.1 . The 15 entries in published SHA256SUMS were matched against GitHub asset SHA-256 digests.
- A repository documentation follow-up adds 26 localized READMEs and three shared English screenshots captured directly from GNOME/Wayland window actor textures through the screenshot portal; these are not desktop crops.
