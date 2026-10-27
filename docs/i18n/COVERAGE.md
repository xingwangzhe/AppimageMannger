# Localization coverage

The application and QuickStart guides support 26 locales. Catalog keys are exact Chinese UI msgids shared by GPUI, Qt, and GTK. Equivalent wording is intentionally normalized to concise, native-language text.

`Error`, `Info`, and `Warning` severity labels remain English. Thirty uncommon diagnostic messages intentionally use English fallback outside Simplified Chinese. All core actions, settings, application details, import, updates, uninstall, and About are localized. Technical names and words shared by a language and English may be identical; they are reported separately from fallback.

Source UI msgids: **204**. Catalog msgids: **215**, including compatibility aliases. Required core keys: **22**. Placeholder mismatches: **0**.

| Locale | Catalog keys | Core keys | Values differing from English | English diagnostic fallbacks | QuickStart |
| --- | ---: | ---: | ---: | ---: | --- |
| en | 215 | 22/22 | 0 | 0 | Yes |
| zh-CN | 215 | 22/22 | 215 | 0 | Yes |
| zh-TW | 215 | 22/22 | 186 | 29 | Yes |
| ja | 215 | 22/22 | 186 | 29 | Yes |
| ko | 215 | 22/22 | 186 | 29 | Yes |
| de | 215 | 22/22 | 181 | 29 | Yes |
| fr | 215 | 22/22 | 177 | 29 | Yes |
| es | 215 | 22/22 | 186 | 29 | Yes |
| pt-BR | 215 | 22/22 | 186 | 29 | Yes |
| ru | 215 | 22/22 | 186 | 29 | Yes |
| it | 215 | 22/22 | 184 | 29 | Yes |
| nl | 215 | 22/22 | 185 | 29 | Yes |
| pl | 215 | 22/22 | 186 | 29 | Yes |
| tr | 215 | 22/22 | 186 | 29 | Yes |
| uk | 215 | 22/22 | 186 | 29 | Yes |
| vi | 215 | 22/22 | 185 | 29 | Yes |
| id | 215 | 22/22 | 186 | 29 | Yes |
| th | 215 | 22/22 | 186 | 29 | Yes |
| ar | 215 | 22/22 | 186 | 29 | Yes |
| hi | 215 | 22/22 | 186 | 29 | Yes |
| bn | 215 | 22/22 | 186 | 29 | Yes |
| fa | 215 | 22/22 | 186 | 29 | Yes |
| he | 215 | 22/22 | 186 | 29 | Yes |
| sv | 215 | 22/22 | 181 | 29 | Yes |
| cs | 215 | 22/22 | 186 | 29 | Yes |
| ro | 215 | 22/22 | 186 | 29 | Yes |

Run `bun run scripts/check-i18n.ts` to verify source coverage, catalog consistency, formatting placeholders, core coverage, and guide contents. Use `--write` to refresh this report. The machine-readable report lists every English fallback and shared value.

## English diagnostic fallback messages

- Theme save failed: {e:#}
- Save failed: {e:#}
- Invalid application list response
- The application directory must be an absolute path
- Invalid application configuration response
- Engine response exceeds the size limit
- Invalid JSON response from the engine: %1
- The engine process did not complete successfully
- Scan failed: {e:#}
- The Rust engine appimage-mannger was not found. Reinstall the application.
- Operation failed
- Operation failed: {e:#}
- The operation timed out. Check the network connection and retry.
- Process communication failed
- The file manager could not open the directory
- Could not switch interface
- Could not start the Rust engine: %1
- Could not start the management engine
- Could not restart: {e}
- Update source save failed: {e:#}
- The directory must not contain control characters
- Invalid directory
- Window toolkit save failed: {e:#}
- The management engine did not return a valid response
- Settings save failed: %1
- Settings save failed: {e:#}
- Enter a valid absolute directory path
- Choose an absolute path
- Configuration load failed: {e:#}

Validation passed without errors.
