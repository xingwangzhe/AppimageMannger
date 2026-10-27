# AppimageMannger agent guide

AppimageMannger is a Linux desktop manager for AppImage applications. The main executable is `appimage-mannger`. Project code is AGPL-3.0-only; retain third-party license and copyright notices.

## Product and architecture

- Default application library: `$HOME/Applications`. This is a deliberate product decision. Never clean, relocate or delete existing applications during routine development or verification.
- Three real frontend toolkits share one Rust engine and the same XDG configuration: Ely/GPUI, Qt 6 Widgets (`native/qt.cpp`), GTK 4/libadwaita (`native/gnome.c`). Qt/GNOME choices launch actual native helpers, not a simulated GPUI theme.
- Prefer Ely components in the GPUI frontend: TitleBar, WindowControls, MenuBar, ListItem, Card, Image, Select, SegmentedControl, TextInput and Button. Use raw GPUI for application/window integration, layout and events where necessary.
- `src/manager.rs` owns filesystem/configuration/desktop integration/Trash; `src/update.rs` owns update discovery and atomic installation; `src/metadata.rs` resolves icons without executing AppImages; `src/frontends.rs` defines JSON RPC and toolkit launch/export; `src/i18n.rs` resolves shared catalogs.
- Configuration is `$XDG_CONFIG_HOME/appimage-mannger/config.json`, falling back to `~/.config`. It persists toolkit, language, theme, library, update repositories, editing drafts, selection and window size/maximized state. Switching toolkit or language saves first and restarts through the engine.

## Working conventions

- Use Bun for JS/TS scripts and routine automation; use Rust/C/C++ tools for the project itself. Do not introduce Python for ordinary processing.
- Use `rg` for searches. Initialize/update CodeGraph with `codegraph init -y .` and `codegraph sync .`; inspect symbols with `codegraph query` or `codegraph explore` when useful. `.codegraph/` is local and ignored.
- Preserve existing work and make focused commits. Publishing or messaging authorization comes from the current user request, not this file.
- System browser is preferred for browsing; leave user-requested tabs open.

## Internationalization

- `resources/i18n/<locale>.json` is the source of shared translation catalogs. The legacy Chinese UI messages are gettext-style message IDs; `en.json` supplies the English fallback. Do not duplicate catalogs in C/C++: `build.rs` generates the native fallback header.
- Application and documentation support 26 locales. Language selection must remain persistent in all three frontends. `system` resolves the current locale; unsupported locales use English.
- Ely accepts only its own small exact locale-tag set. Use `i18n::ely_i18n` to map application locales safely and override shared Browse/Choose copy; unsupported Ely internal strings use English fallback. Never pass arbitrary application locale codes directly to `I18n::new`.
- Preserve `{name}`, `{}`, `%1`, `%s`, and other format placeholders exactly. Run `bun run scripts/check-i18n.ts` after changing visible text or catalogs.
- Diagnostic level names (`Error`, `Info`, `Warning`, `Debug`, `Trace`) and engine errors stay English. Do not translate managed third-party application names, paths, repository identifiers, command names or logs as if they were UI labels. This manager's own display name is localized in all 26 catalogs and desktop Name entries; retain stable executable/package/configuration identifiers.
- Maintain native-name language choices, RTL behavior for Arabic/Hebrew/Persian, and localized usage docs under `docs/i18n/`.

## Safety invariants

- Scan/metadata extraction never runs or mounts an AppImage. Embedded icons use bounded `unsquashfs -cat`/listing and a path/mtime/size-keyed cache.
- Imports keep the source. Uninstall moves the managed executable and matching launchers to freedesktop Trash with restoration metadata; personal application data stays in place.
- Updates use HTTPS, stable releases, architecture checks, bounded transfers, upstream SHA-256 where provided, and atomic replacement with a backup/rollback. A failed update must preserve the existing application.
- Keep filesystem boundaries and symlink checks. Use argument arrays for subprocesses, never shell-interpolate paths or JSON.
- Native frontend requests are asynchronous and serialized; do not let stale scan/save responses overwrite new edits. Save configuration before closing or restarting.

## Validation and releases

Read [build and release guidance](docs/BUILD-AND-RELEASE.md) before packaging. A normal verification pass includes formatting, strict Clippy, Rust tests, i18n checks, real toolkit compilation, local package inspection, and a local installed-window smoke test. Test imports/uninstall against isolated fixtures, not the user's existing applications.

For a user-authorized release: build the local deb, install through `pkexec` when needed, open the installed program, verify persistence after restart and actual icons/window movement, then push a version-matching tag. CI builds x86_64 and aarch64 packages on native current runners and publishes only after required jobs succeed. Inspect the actual assets/checksums before declaring a release complete. AUR recipe archives are distinct from installable `.pkg.tar.zst` files and do not imply an AUR service publication.

## Repository skills

- [Ely/GPUI interface work](skills/ely-gpui-native-ui/SKILL.md)
- [AppImage engine and project logic](skills/appimage-manager-core/SKILL.md)
- [Native Linux build, verification and release](skills/linux-native-release/SKILL.md)

The 1.0.0 baseline lives in [the original release record](docs/RELEASE-1.0.0.md). The replacement-import, desktop integration, resizable-layout and localized-name fixes are recorded in [1.0.1](docs/RELEASE-1.0.1.md).
