# 1.0.3 release record

## Scope

This is a rebuild release of the 1.0.2 application behavior. It advances the package and CLI version to 1.0.3 and documents installing the published CI artifact when byte-for-byte identity with GitHub Release downloads is required. No application behavior, translations, dependencies or library contents are intentionally changed.

Local builds and GitHub CI builds may have different binary hashes even when their version and source behavior match. The final local installation for this release must use the published amd64 DEB, with the engine and both native helpers compared against the extracted download.

## Validation

- Formatting, strict Clippy, all 36 release-mode correctness tests and the 26-language catalog/README checks passed. The separate ignored translation benchmark was not rerun for this version-only rebuild.
- The local release build compiled the Rust engine and both real native helpers. The amd64 DEB was inspected and installed successfully through pkexec; dpkg and the installed CLI report 1.0.3. Local verification DEB SHA-256: `609104808b366230e06f62a4b0375f44f0ecd6e36bf458ebaad0de202288917a`.
- The installed CLI scanned an isolated inert fixture successfully. The installed GPUI window rendered that fixture and its icon. Further local click/interaction testing is reserved for the user at their request.
- Published CI results, release artifact digests and the final installation comparison will be recorded after they have been verified.
