# 1.0.0 handoff

This record describes the completed scope and records actual verification evidence. Entries awaiting a final build/release result are marked pending until checked.

## Product

- Shared Rust engine with real Ely/GPUI, Qt 6 Widgets and GTK 4/libadwaita frontends.
- Default application library `~/Applications`; import, desktop integration, launch, explicit update discovery/install and recoverable uninstall.
- Actual theme/embedded AppImage icons, without running/mounting the application to inspect it.
- Draggable captions and native window controls; persistent toolkit, theme, language, directory/repositories/drafts, selection and window dimensions.
- 26 application catalogs and localized usage guides. English diagnostic levels/engine errors are preserved.
- AGPL-3.0-only for project code, with dependency notices retained.

## Documentation and continued work

[AGENTS.md](../AGENTS.md) routes future agents to the actual architecture and verification rules. Repository skills cover [Ely/GPUI UI work](../skills/ely-gpui-native-ui/SKILL.md), [AppImage core logic](../skills/appimage-manager-core/SKILL.md), and [native Linux release work](../skills/linux-native-release/SKILL.md). The language catalog checker and coverage reports make the translation baseline reviewable.

## Verified during implementation

- Original local AppImageLauncher was removed before development; existing GpgFrontend under Applications was preserved.
- The initial local deb was installed via pkexec and the native GPUI window was opened. Its original caption/persistence/icon omissions were reproduced and corrected in the subsequent implementation.
- Backend/config/icon regression suite: 23 tests passed before final i18n integration.
- Qt and GTK real helper compilation succeeded; the Qt helper also passed an isolated asynchronous save/resize/close fixture test.
- i18n checker: 26 catalogs × 215 keys, 204 source message IDs, 22/22 core checks per language, zero placeholder mismatches; 26 QuickStarts validated. Low-frequency diagnostic English fallback is explicitly reported in the coverage files.

## Final release verification

Local build and installation evidence is recorded below. Tagged CI publication remains pending until the workflow completes; actual release URLs and artifact counts are added afterward.

### Local 1.0.0 result (2026-10-27)

- Release binary built successfully with the native Qt and GTK helpers and bundled 26-language catalogs.
- Strict Clippy passed; 28 Rust regression tests passed before the owner requested stopping additional manual tests.
- Translation validation passed: 26 locales, 215 message keys, 204 source msgids, all core checks and QuickStarts, no placeholder mismatches.
- The release DEB was generated at `dist/appimage-mannger_1.0.0_amd64.deb` and installed using `pkexec apt-get install`. The package manager confirmed `ii 1.0.0`.
- `/usr/bin/appimage-mannger` was started after installation. The desktop launcher continues to use that installed executable.
- English GPUI rendering, actual GpgFrontend icons and saving the maximized flag were observed during the earlier isolated configuration session. At the owner's request, further manual GUI testing was stopped; an exhaustive 26-language × 3-toolkit visual/restart matrix is not claimed.
- The application still defaults to `~/Applications`. No update or uninstall was performed on the user's existing GpgFrontend as part of final verification.

The stable Git tag is `v1.0.0`. CI publication results and asset URLs follow after the tagged workflow completes.
