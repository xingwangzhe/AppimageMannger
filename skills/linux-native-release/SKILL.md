---
name: linux-native-release
description: Build, inspect, locally install and release AppimageMannger's real GPUI, Qt and GTK Linux application packages for x86_64 and aarch64.
---

Read [AGENTS.md](../../AGENTS.md), [build and release](../../docs/BUILD-AND-RELEASE.md), and the current workflow. User authorization for publishing must come from the current conversation; this skill does not authorize a new release by itself.

Keep the version in Cargo.toml, Cargo.lock, CLI output and the release tag consistent. The project is AGPL-3.0-only; preserve the dependency notices and native toolkit license texts.

Use Bun packaging scripts and native Rust/C/C++ compilers. All real frontends must compile: `build.rs` uses Qt6Widgets, GTK4/libadwaita and json-glib, exports their helpers, and generates the shared native translation fallback. A deb dependency scan must include the helpers, not only the Rust engine; QPA/Wayland plugins and dynamically loaded Vulkan/icon tools need explicit attention.

Before publishing, run appropriate format/lint/test/i18n checks, build the release binary, generate the local deb, inspect its contents/dependencies, and install with the user-authorized privilege mechanism (`pkexec` on this machine). Open `/usr/bin/appimage-mannger` and verify real windows for the requested toolkits, saved preferences after restart, actual application icons and caption movement. Keep the user's applications and unrelated windows intact.

CI uses native x86_64/aarch64 current Ubuntu runners and current stable Rust/Bun. Confirm current runner/tool versions when updating the workflow. A matching tag drives release packaging; inspect job results and actual release assets instead of equating a pushed tag with a successful release.

DEB, RPM, AppImage, portable archive and pacman `.pkg.tar.zst` are binary outputs. AUR PKGBUILD/.SRCINFO archives are recipes and do not require or imply publication to aur.archlinux.org. Verify SHA256SUMS and document the actual glibc/toolkit compatibility baseline. Update the handoff record only with results that were verified.
