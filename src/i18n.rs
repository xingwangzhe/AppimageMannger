//! Shared gettext-style catalogs used by Ely, Qt and GTK frontends.
use regex::{Regex, RegexSet};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    sync::{OnceLock, RwLock},
};
type Catalogs = BTreeMap<String, BTreeMap<String, String>>;
const LOCALES: [(&str, &str); 26] = [
    ("en", "English"),
    ("zh-CN", "简体中文"),
    ("zh-TW", "繁體中文"),
    ("ja", "日本語"),
    ("ko", "한국어"),
    ("de", "Deutsch"),
    ("fr", "Français"),
    ("es", "Español"),
    ("pt-BR", "Português (Brasil)"),
    ("ru", "Русский"),
    ("it", "Italiano"),
    ("nl", "Nederlands"),
    ("pl", "Polski"),
    ("tr", "Türkçe"),
    ("uk", "Українська"),
    ("vi", "Tiếng Việt"),
    ("id", "Bahasa Indonesia"),
    ("th", "ไทย"),
    ("ar", "العربية"),
    ("hi", "हिन्दी"),
    ("bn", "বাংলা"),
    ("fa", "فارسی"),
    ("he", "עברית"),
    ("sv", "Svenska"),
    ("cs", "Čeština"),
    ("ro", "Română"),
];
fn catalogs() -> &'static Catalogs {
    static DATA: OnceLock<Catalogs> = OnceLock::new();
    DATA.get_or_init(|| {
        serde_json::from_str(include_str!(concat!(env!("OUT_DIR"), "/catalogs.json")))
            .expect("Invalid bundled translation catalogs")
    })
}
fn active() -> &'static RwLock<&'static str> {
    static ACTIVE: OnceLock<RwLock<&'static str>> = OnceLock::new();
    ACTIVE.get_or_init(|| RwLock::new("en"))
}
pub fn locale_list() -> Value {
    json!(
        LOCALES
            .iter()
            .map(|(code, name)| json!({"code":code,"name":name}))
            .collect::<Vec<_>>()
    )
}
pub fn resolve(language: &str) -> String {
    resolve_code(language).into()
}
fn resolve_code(language: &str) -> &'static str {
    let raw = if language.is_empty() || language == "system" {
        std::env::var("LC_ALL")
            .ok()
            .filter(|s| !s.is_empty())
            .or_else(|| std::env::var("LC_MESSAGES").ok().filter(|s| !s.is_empty()))
            .or_else(|| std::env::var("LANG").ok())
            .unwrap_or_else(|| "en".into())
    } else {
        language.into()
    };
    let raw = raw
        .split(['.', ':', '@'])
        .next()
        .unwrap_or("en")
        .replace('_', "-");
    if let Some((code, _)) = LOCALES.iter().find(|(c, _)| c.eq_ignore_ascii_case(&raw)) {
        return code;
    }
    let lower = raw.to_ascii_lowercase();
    if lower.starts_with("zh-hant") || lower.starts_with("zh-tw") || lower.starts_with("zh-hk") {
        return "zh-TW";
    }
    let base = lower.split('-').next().unwrap_or("en");
    if base == "zh" {
        return "zh-CN";
    }
    if base == "pt" {
        return "pt-BR";
    }
    LOCALES
        .iter()
        .find(|(code, _)| *code == base)
        .map(|(code, _)| *code)
        .unwrap_or("en")
}
pub fn set_locale(language: &str) {
    *active().write().expect("Locale lock poisoned") = resolve_code(language);
}
pub fn current_locale() -> &'static str {
    *active().read().expect("Locale lock poisoned")
}
pub fn is_rtl() -> bool {
    matches!(current_locale(), "ar" | "fa" | "he")
}
pub fn translations(language: &str) -> Value {
    let mut merged = catalogs().get("en").cloned().unwrap_or_default();
    if let Some(local) = catalogs().get(&resolve(language)) {
        merged.extend(local.clone());
    }
    json!(merged)
}
struct Pattern {
    regex: Regex,
    key: String,
    tokens: Vec<String>,
}
struct Patterns {
    candidates: RegexSet,
    entries: Vec<Pattern>,
}
fn patterns() -> &'static Patterns {
    static PATTERNS: OnceLock<Patterns> = OnceLock::new();
    PATTERNS.get_or_init(|| {
        let fields = Regex::new(r"\{[^}]*\}").unwrap();
        let entries: Vec<_> = catalogs()
            .get("en")
            .into_iter()
            .flat_map(|m| m.keys())
            .filter_map(|key| {
                let found: Vec<_> = fields.find_iter(key).collect();
                if found.is_empty() {
                    return None;
                }
                let mut expression = String::from("(?s)^");
                let mut last = 0;
                let mut tokens = Vec::new();
                for m in found {
                    expression.push_str(&regex::escape(&key[last..m.start()]));
                    expression.push_str("(.*?)");
                    tokens.push(m.as_str().into());
                    last = m.end();
                }
                expression.push_str(&regex::escape(&key[last..]));
                expression.push('$');
                Some(Pattern {
                    regex: Regex::new(&expression).ok()?,
                    key: key.clone(),
                    tokens,
                })
            })
            .collect();
        Patterns {
            candidates: RegexSet::new(entries.iter().map(|p| p.regex.as_str()))
                .expect("Invalid bundled translation patterns"),
            entries,
        }
    })
}
fn english_keys() -> &'static BTreeMap<&'static str, &'static str> {
    static KEYS: OnceLock<BTreeMap<&'static str, &'static str>> = OnceLock::new();
    KEYS.get_or_init(|| {
        let mut keys = BTreeMap::new();
        if let Some(english) = catalogs().get("en") {
            for (key, value) in english {
                // Keep the first key in catalog order when English values repeat.
                keys.entry(value.as_str()).or_insert(key.as_str());
            }
        }
        keys
    })
}
fn lookup(key: &str, language: &str) -> Option<&'static str> {
    catalogs()
        .get(language)
        .and_then(|m| m.get(key))
        .or_else(|| catalogs().get("en").and_then(|m| m.get(key)))
        .map(String::as_str)
}
pub fn tr(source: impl AsRef<str>) -> String {
    translate_resolved(source.as_ref(), current_locale())
}
#[cfg(test)]
pub fn translate(source: &str, language: &str) -> String {
    translate_resolved(source, resolve_code(language))
}
fn translate_resolved(source: &str, language: &str) -> String {
    if matches!(source, "Error" | "Info" | "Warning" | "Debug" | "Trace") {
        return source.into();
    }
    if let Some(text) = lookup(source, language) {
        return text.into();
    }
    if let Some(key) = english_keys().get(source)
        && let Some(text) = lookup(key, language)
    {
        return text.into();
    }
    let patterns = patterns();
    for index in patterns.candidates.matches(source) {
        let pattern = &patterns.entries[index];
        if let Some(captures) = pattern.regex.captures(source)
            && let Some(template) = lookup(&pattern.key, language)
        {
            let mut out = template.to_owned();
            for (i, token) in pattern.tokens.iter().enumerate() {
                out = out.replacen(token, captures.get(i + 1).map_or("", |v| v.as_str()), 1);
            }
            return out;
        }
    }
    source.into()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn language_resolution() {
        assert_eq!(resolve("zh_HK.UTF-8"), "zh-TW");
        assert_eq!(resolve("pt_PT.UTF-8"), "pt-BR");
        assert_eq!(resolve("unknown"), "en");
        assert_eq!(resolve("de-DE"), "de");
    }
    #[test]
    fn diagnostics_are_english() {
        for (c, _) in LOCALES {
            for level in ["Error", "Info", "Warning"] {
                assert_eq!(translate(level, c), level);
            }
        }
    }
    #[test]
    fn supported_language_count() {
        assert_eq!(LOCALES.len(), 26);
    }
}

/// Ely currently accepts a small exact BCP-47 tag set. Application catalogs
/// remain independent, so unsupported Ely builtins safely use English.
pub fn ely_i18n(language: &str) -> ely_core::i18n::I18n {
    let lang = resolve(language);
    let tag = match lang.as_str() {
        "de" => "de-DE",
        "fr" => "fr-FR",
        "ja" => "ja-JP",
        "zh-CN" => "zh-CN",
        "he" => "he-IL",
        _ => "en-US",
    };
    // Bundled catalog strings already live for the process lifetime.
    let browse = lookup("浏览…", &lang).unwrap_or("Browse…");
    let choose = lookup("选择…", &lang).unwrap_or("Choose…");
    ely_core::i18n::I18n::new(tag)
        .catalog(tag, &[("input.browse", browse), ("input.choose", choose)])
}
#[cfg(test)]
mod integration_tests {
    use super::*;
    // Original lookup algorithm, retained as a behavior oracle and benchmark.
    fn reference_translate(source: &str, language: &str) -> String {
        if matches!(source, "Error" | "Info" | "Warning" | "Debug" | "Trace") {
            return source.into();
        }
        let lang = resolve(language);
        let en = catalogs().get("en");
        let selected = catalogs().get(&lang);
        let lookup = |key: &str| {
            selected
                .and_then(|m| m.get(key))
                .or_else(|| en.and_then(|m| m.get(key)))
        };
        if let Some(text) = lookup(source) {
            return text.clone();
        }
        if let Some((key, _)) =
            en.and_then(|m| m.iter().find(|(_, english)| english.as_str() == source))
            && let Some(text) = lookup(key)
        {
            return text.clone();
        }
        for pattern in &patterns().entries {
            if let Some(captures) = pattern.regex.captures(source)
                && let Some(template) = lookup(&pattern.key)
            {
                let mut out = template.clone();
                for (i, token) in pattern.tokens.iter().enumerate() {
                    out = out.replacen(token, captures.get(i + 1).map_or("", |v| v.as_str()), 1);
                }
                return out;
            }
        }
        source.into()
    }
    #[test]
    fn indexed_translation_matches_original_in_every_locale() {
        let mut sources = vec![
            "Error".to_owned(),
            "Some unknown application name".to_owned(),
            "已导入 Example\n第二行".to_owned(),
        ];
        for (key, value) in &catalogs()["en"] {
            sources.push(key.clone());
            sources.push(value.clone());
        }
        for pattern in &patterns().entries {
            let mut source = pattern.key.clone();
            for token in &pattern.tokens {
                source = source.replacen(token, "示例 Example 123", 1);
            }
            sources.push(source);
        }
        for (language, _) in LOCALES {
            for source in &sources {
                assert_eq!(
                    translate(source, language),
                    reference_translate(source, language),
                    "{language}: {source}"
                );
            }
        }
    }
    #[test]
    #[ignore = "manual release-mode performance measurement"]
    fn translation_benchmark() {
        use std::{hint::black_box, time::Instant};
        let sources = [
            "准备就绪",
            "Imported Example",
            "已导入 Example",
            "Unknown AppImage",
        ];
        let run = |translate: fn(&str, &str) -> String| {
            let start = Instant::now();
            for _ in 0..25_000 {
                for source in sources {
                    black_box(translate(black_box(source), black_box("de")));
                }
            }
            start.elapsed().as_secs_f64() * 1000.0
        };
        for source in sources {
            black_box(translate(source, "de"));
            black_box(reference_translate(source, "de"));
        }
        let mut old = Vec::new();
        let mut new = Vec::new();
        for i in 0..7 {
            if i % 2 == 0 {
                old.push(run(reference_translate));
                new.push(run(translate));
            } else {
                new.push(run(translate));
                old.push(run(reference_translate));
            }
        }
        old.sort_by(f64::total_cmp);
        new.sort_by(f64::total_cmp);
        println!(
            "{}",
            json!({"calls_per_sample":100_000,"samples":7,"baseline_ms":old,"optimized_ms":new,"speedup":old[3]/new[3]})
        );
    }
    #[test]
    fn every_application_locale_initializes_ely() {
        for (code, _) in LOCALES {
            let _ = ely_i18n(code);
        }
    }
    #[test]
    fn dynamic_strings_keep_values() {
        assert_eq!(translate("已导入 Example", "en"), "Imported Example");
        assert_eq!(
            translate("已导入 2 个应用", "en"),
            "Imported 2 applications"
        );
    }
}
