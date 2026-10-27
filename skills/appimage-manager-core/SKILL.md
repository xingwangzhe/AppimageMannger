---
name: appimage-manager-core
description: Modify or diagnose AppimageMannger's Rust application library, metadata/icon inspection, desktop integration, JSON RPC, configuration and update transactions.
---

Read [AGENTS.md](../../AGENTS.md) and [architecture](../../docs/ARCHITECTURE.md), then use CodeGraph or `rg` to find the relevant public API.

The default library is `$HOME/Applications`. Preserve its existing applications while developing. Use isolated directories and XDG configuration roots for destructive-operation fixtures.

`manager.rs` owns Config, scan/inspect, imports, launchers and Trash. `update.rs` owns release discovery and the atomic install/rollback flow. `metadata.rs` reads theme/embedded icons without executing the AppImage. `frontends.rs` exposes the shared engine to Qt and GTK through `--rpc`; keep the engine's `ok/data/error` contract stable and return English diagnostics.

Keep these invariants observable:

- Inspection does not execute or mount applications. ELF/SquashFS parsing has explicit bounds and subprocess time/output budgets.
- Import preserves its source and handles duplicate names explicitly.
- Uninstall respects the configured directory boundary, rejects unsafe symlinks and preserves restorable Trash metadata for the executable and matching launchers.
- Download or verification failure leaves the existing executable intact. Successful updates retain rollback data, install atomically and persist a verifiable current version.
- Configuration writes are atomic and survive a fresh process. Old configurations load with serde defaults; toolkit, theme, language and editing drafts do not overwrite each other.

Tests should exercise real temporary files and transactions: valid/invalid headers, embedded icon fixtures, out-of-library paths, multiple launcher cleanup, downloaded format/architecture/digest failures, success and rollback, and save/reload. A UI toast is not evidence that an update transaction or package installation succeeded.

Use argument arrays for every subprocess and keep filesystem paths out of shell source. Refer to [the release record](../../docs/RELEASE-1.0.0.md) for the validated baseline and known limitations.
