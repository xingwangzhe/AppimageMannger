//! Shared gettext-style catalogs used by Ely, Qt and GTK frontends.
use regex::Regex;
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
fn active() -> &'static RwLock<String> {
    static ACTIVE: OnceLock<RwLock<String>> = OnceLock::new();
    ACTIVE.get_or_init(|| RwLock::new("en".into()))
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
        return (*code).into();
    }
    let lower = raw.to_ascii_lowercase();
    if lower.starts_with("zh-hant") || lower.starts_with("zh-tw") || lower.starts_with("zh-hk") {
        return "zh-TW".into();
    }
    let base = lower.split('-').next().unwrap_or("en");
    if base == "zh" {
        return "zh-CN".into();
    }
    if base == "pt" {
        return "pt-BR".into();
    }
    LOCALES
        .iter()
        .find(|(code, _)| *code == base)
        .map(|(code, _)| (*code).into())
        .unwrap_or_else(|| "en".into())
}
pub fn set_locale(language: &str) {
    *active().write().expect("Locale lock poisoned") = resolve(language);
}
pub fn current_locale() -> String {
    active().read().expect("Locale lock poisoned").clone()
}
pub fn is_rtl() -> bool {
    matches!(current_locale().as_str(), "ar" | "fa" | "he")
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
fn patterns() -> &'static Vec<Pattern> {
    static PATTERNS: OnceLock<Vec<Pattern>> = OnceLock::new();
    PATTERNS.get_or_init(|| {
        let fields = Regex::new(r"\{[^}]*\}").unwrap();
        catalogs()
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
            .collect()
    })
}
pub fn tr(source: impl AsRef<str>) -> String {
    translate(source.as_ref(), &current_locale())
}
pub fn translate(source: &str, language: &str) -> String {
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
    for pattern in patterns() {
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
    let browse: &'static str = Box::leak(translate("浏览…", &lang).into_boxed_str());
    let choose: &'static str = Box::leak(translate("选择…", &lang).into_boxed_str());
    ely_core::i18n::I18n::new(tag)
        .catalog(tag, &[("input.browse", browse), ("input.choose", choose)])
}
#[cfg(test)]
mod integration_tests {
    use super::*;
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
