# Build and release

AppimageMannger uses Rust plus native Qt 6 Widgets and GTK 4/libadwaita helpers. The default frontend uses Ely Core/GPUI. JavaScript/TypeScript tooling runs with Bun.

On Ubuntu 26.04 install build tools and native development dependencies:

```sh
sudo apt install build-essential pkg-config clang cmake libfontconfig-dev libxkbcommon-x11-dev libwayland-dev libvulkan-dev qt6-base-dev libgtk-4-dev libadwaita-1-dev libjson-glib-dev squashfs-tools desktop-file-utils
```

Validation and local packaging:

```sh
bun run scripts/check-i18n.ts
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
cargo build --release --locked
bun run scripts/package.ts --formats deb
pkexec apt-get install -y "$PWD"/dist/appimage-mannger_1.0.0_amd64.deb
/usr/bin/appimage-mannger
```

`build.rs` compiles both native helpers and generates `i18n_fallback.h` from the English catalog. The Rust launcher selects the configured toolkit; helpers call the engine through asynchronous JSON RPC. Native packages install helpers under `/usr/lib/appimage-mannger`. Development/portable launches can use embedded helpers in a private user cache.

Use a distinct XDG_CONFIG_HOME for persistence tests when the user's active preferences should remain untouched. Validate at least one language switch, toolkit switch, theme change, window close/reopen, actual icon rendering, and the draggable caption. Scan does not execute the managed applications. Use temporary fixtures to test imports/uninstall and compare existing application checksums afterward.

Full native packaging additionally needs `rpm` (rpmbuild), `zstd`, `libarchive-tools` (bsdtar), `file`, and `dpkg-dev`. Install these before requesting all formats. The native architecture scripts do not cross-build the other architecture.

Full native packaging uses:

```sh
bun run scripts/package.ts --formats deb,rpm,appimage,tar,arch
# The following assembly is run AFTER both native architecture archives are present.
# CI downloads x86_64 and aarch64 artifacts before this step.
bun run scripts/package-aur.ts
bun run scripts/package-checksums.ts
(cd dist && sha256sum -c SHA256SUMS)
```

AppImage creation resolves an official appimagetool release and may need the existing gh credential to avoid anonymous API limits. Package/AppDir assembly includes native toolkit libraries/plugins/schemas and third-party notices. System libc and GPU drivers remain a compatibility boundary. Runtime dependency scans must include the Rust engine and both helper ELF files.

The workflow builds x86_64 and aarch64 on native current runners. Source and binary AUR recipes are release artifacts; a real pacman `.pkg.tar.zst` is a separate installable archive. There is no automatic AUR service submission.

When release publication is authorized, commit validated sources, push the branch, and push an exact `v<version>` tag. Required CI jobs run checks, build/package, inspect/install smoke tests, assemble translated documentation/source archives, and verify checksums before publishing. Do not declare success until the workflow and actual release assets are checked. Record verified local/CI results in [the release handoff](RELEASE-1.0.0.md).
