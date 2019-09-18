mod tables;

use unic_langid::LanguageIdentifier;

fn lookup_binary<K: Ord, T: Copy>(key: K, data: &[(K, T)]) -> Option<T> {
    data.binary_search_by(|(k, _)| k.cmp(&key))
        .ok()
        .map(|i| data[i].1)
}

pub fn add_likely_subtags(langid: &LanguageIdentifier) -> Option<LanguageIdentifier> {
    let lang = langid.get_language();
    let lang = if lang == "und" { None } else { Some(lang) };
    let region = langid.get_region();
    let script = langid.get_script();

    match (lang, region, script) {
        (Some(l), None, None) => {
            let lookup = lookup_binary(l, tables::LANG_ONLY);
            lookup.map(|l| l.parse().unwrap())
        }
        (Some(_), Some(_), None) => {
            let lookup = lookup_binary(langid.to_string().as_str(), tables::LANG_REGION);
            lookup.map(|l| l.parse().unwrap())
        }
        (Some(_), None, Some(_)) => {
            let lookup = lookup_binary(langid.to_string().as_str(), tables::LANG_SCRIPT);
            lookup.map(|l| l.parse().unwrap())
        }
        (None, Some(_), Some(_)) => {
            let lookup = lookup_binary(langid.to_string().as_str(), tables::SCRIPT_REGION);
            lookup.map(|l| l.parse().unwrap())
        }
        (None, None, Some(r)) => {
            let lookup = lookup_binary(r, tables::REGION_ONLY);
            lookup.map(|l| l.parse().unwrap())
        }
        (None, Some(s), None) => {
            let lookup = lookup_binary(s, tables::SCRIPT_ONLY);
            lookup.map(|l| l.parse().unwrap())
        }
        _ => None,
    }
}
