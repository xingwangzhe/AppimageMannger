# Localization coverage

The application and QuickStart guides support 26 locales. Catalog keys are exact Chinese UI msgids shared by GPUI, Qt, and GTK. Equivalent wording is intentionally normalized to concise, native-language text.

`Error`, `Info`, and `Warning` severity labels remain English. Thirty uncommon diagnostic messages intentionally use English fallback outside Simplified Chinese. All core actions, settings, application details, import, updates, uninstall, and About are localized. Technical names and words shared by a language and English may be identical; they are reported separately from fallback.

Source UI msgids: **206**. Catalog msgids: **220**, including compatibility aliases. Required core keys: **22**. Placeholder mismatches: **0**.

| Locale | Catalog keys | Core keys | Values differing from English | English diagnostic fallbacks | QuickStart |
| --- | ---: | ---: | ---: | ---: | --- |
| en | 220 | 22/22 | 0 | 0 | Yes |
| zh-CN | 220 | 22/22 | 220 | 0 | Yes |
| zh-TW | 220 | 22/22 | 191 | 29 | Yes |
| ja | 220 | 22/22 | 191 | 29 | Yes |
| ko | 220 | 22/22 | 191 | 29 | Yes |
| de | 220 | 22/22 | 186 | 29 | Yes |
| fr | 220 | 22/22 | 182 | 29 | Yes |
| es | 220 | 22/22 | 191 | 29 | Yes |
| pt-BR | 220 | 22/22 | 191 | 29 | Yes |
| ru | 220 | 22/22 | 191 | 29 | Yes |
| it | 220 | 22/22 | 188 | 29 | Yes |
| nl | 220 | 22/22 | 190 | 29 | Yes |
| pl | 220 | 22/22 | 191 | 29 | Yes |
| tr | 220 | 22/22 | 191 | 29 | Yes |
| uk | 220 | 22/22 | 191 | 29 | Yes |
| vi | 220 | 22/22 | 189 | 29 | Yes |
| id | 220 | 22/22 | 190 | 29 | Yes |
| th | 220 | 22/22 | 191 | 29 | Yes |
| ar | 220 | 22/22 | 191 | 29 | Yes |
| hi | 220 | 22/22 | 191 | 29 | Yes |
| bn | 220 | 22/22 | 191 | 29 | Yes |
| fa | 220 | 22/22 | 191 | 29 | Yes |
| he | 220 | 22/22 | 191 | 29 | Yes |
| sv | 220 | 22/22 | 186 | 29 | Yes |
| cs | 220 | 22/22 | 191 | 29 | Yes |
| ro | 220 | 22/22 | 191 | 29 | Yes |

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
