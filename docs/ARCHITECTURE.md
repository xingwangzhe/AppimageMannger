# Architecture

AppimageMannger is a Rust 2024 Linux application using Ely Core (the general UI component crate of Ely GPUI Components) and GPUI's native X11/Wayland platform. Ely is pinned at a reviewed upstream commit; Cargo.lock records dependency resolution. No browser/webview is involved.

- `src/ui.rs`: menus, keyboard bindings, master/detail library, settings, import, update and uninstall confirmations. Blocking work runs on worker threads and returns to the GPUI entity through a oneshot channel.
- `src/manager.rs`: XDG configuration, default `$HOME/Applications`, ELF header/section parsing, metadata and desktop integration, import and freedesktop Trash uninstall.
- `src/update.rs`: GitHub stable release discovery, architecture selection, bounded HTTPS download, optional upstream SHA-256 verification, atomic replacement and rollback.
- `scripts/package*.ts`: Bun-driven native deb/rpm/AppImage/archive builds and AUR source/binary recipes. Package installation uses normal system packaging; the application runs as the logged-in user.
- `.github/workflows/linux.yml`: native x86_64/aarch64 tests, release packaging, install smoke tests, checksums and release publishing.

Configuration lives under `$XDG_CONFIG_HOME/appimage-mannger/config.json`; missing XDG variables use the standard home directories. Applications live in `~/Applications` by default. Changing this setting selects another library and does not move files. Scan never launches an AppImage. Import copies instead of moving the source file. Uninstall keeps user application data and moves the executable to Trash. The manager never needs a persistent privileged daemon.

Updates need embedded `gh-releases-zsync` information, a supported desktop repository field, or an explicitly entered GitHub owner/repository. Unknown or non-semver current versions produce an explanatory result instead of guessing whether a download is newer. Non-GitHub update protocols are displayed as metadata but are not silently treated as supported.

Latest GitHub stable releases exclude prereleases. Both the asset name and downloaded ELF architecture are checked. A successful update retains the old binary in the manager's XDG data backup directory. An upstream GitHub SHA-256 digest is checked when provided; without one the app can validate format and transfer size but cannot claim independent publisher authenticity.

CodeGraph index is local in `.codegraph/` and ignored by Git. Run `codegraph init -y .` after cloning and `codegraph sync .` after edits.
