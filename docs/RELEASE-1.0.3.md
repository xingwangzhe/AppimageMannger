# 1.0.3 release record

## Scope

This is a rebuild release of the 1.0.2 application behavior. It advances the package and CLI version to 1.0.3 and documents installing the published CI artifact when byte-for-byte identity with GitHub Release downloads is required. No application behavior, translations, dependencies or library contents are intentionally changed.

Local builds and GitHub CI builds may have different binary hashes even when their version and source behavior match. The final local installation for this release must use the published amd64 DEB, with the engine and both native helpers compared against the extracted download.

## Validation

- Formatting, strict Clippy, all 36 release-mode correctness tests and the 26-language catalog/README checks passed. The separate ignored translation benchmark was not rerun for this version-only rebuild.
- The local release build compiled the Rust engine and both real native helpers. The amd64 DEB was inspected and installed successfully through pkexec; dpkg and the installed CLI report 1.0.3. Local verification DEB SHA-256: `609104808b366230e06f62a4b0375f44f0ecd6e36bf458ebaad0de202288917a`.
- The installed CLI scanned an isolated inert fixture successfully. The installed GPUI window rendered that fixture and its icon. Further local click/interaction testing is reserved for the user at their request.

## Published release

- Stable [v1.0.3](https://github.com/xingwangzhe/AppimageMannger/releases/tag/v1.0.3) targets signed tag commit `9323db546bc5265a754ee0cd84e4d3b63b81e899`.
- The [tagged workflow](https://github.com/xingwangzhe/AppimageMannger/actions/runs/40481902829) passed both native architectures and release assembly. Its first x86_64 attempt was cancelled after an Ubuntu mirror download stalled; rerunning the unfinished jobs succeeded and retained the successful aarch64 result. The same-commit [main workflow](https://github.com/xingwangzhe/AppimageMannger/actions/runs/40481898567) also passed.
- The release contains 16 assets. All 15 `SHA256SUMS` entries match the GitHub asset SHA-256 digests. The downloaded checksum manifest, both architecture DEBs and both build-info files were independently hashed and matched those digests.
- Published amd64 DEB SHA-256: `1470bd3c1e6d7a8a517d3936f65ff96b615c610f72eb58d61ee50876e13e8af7`.
- Reinstalled the downloaded published amd64 DEB with `pkexec apt-get install --reinstall`. dpkg reports `install ok installed`, version 1.0.3, architecture amd64. All 960 regular files in the extracted package match their installed counterparts by SHA-256. The existing user AppImage hashes were unchanged.

| Installed file | SHA-256, identical to the published DEB |
| --- | --- |
| `/usr/bin/appimage-mannger` | `ce8e561d745c730d33de869e60b55910f359f790eecc13111c1bc74ca72ad5a3` |
| `/usr/lib/appimage-mannger/frontend-qt` | `1773d7ae8a2da1f371e241bb5214d8562afba7235bec48db481206b397fc15d6` |
| `/usr/lib/appimage-mannger/frontend-gnome` | `9a7c382ae568de2deb6914550ff32b4a7abe8a0d623d1b8f90cf28e01df9a474` |

Further local click/interaction testing remains with the user. The final comparison above establishes installed-file identity with the published CI build; it does not claim a new interactive test of that build.
