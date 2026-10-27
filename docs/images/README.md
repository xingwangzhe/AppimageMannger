# README screenshots

These are unmodified GNOME/Wayland compositor window-actor captures of the installed AppImage Manager 1.0.1 on Linux, taken on 2026-10-27.

- `ely-gpui-en.png`: Ely/GPUI frontend.
- `qt-en.png`: real Qt 6 Widgets frontend.
- `gnome-en.png`: real GNOME / GTK 4 / libadwaita frontend.

All three use English, the dark theme, a 1440 × 1040 window and the same isolated copy of GpgFrontend 2.2.3. No AppImage was executed for capture. User preferences and application files were kept separate from the capture configuration. Every localized README references these same English screenshots.

Capture used XDG Desktop Portal Screenshot with interactive=true, followed by GNOME 50 built-in screenshot UI in Window mode (W, Enter). The committed files come from the selected window actor texture; they are not crops of desktop screenshots. Window decorations may add transparent margins: Ely/GPUI is 1440 × 1040, Qt is 1460 × 1060, and GNOME is 1490 × 1090.

Localized READMEs are maintained with `bun run scripts/generate-readmes.ts`; their installation and usage sections reuse the corresponding QuickStart under `docs/i18n/`.
