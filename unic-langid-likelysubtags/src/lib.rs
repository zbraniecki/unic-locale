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
        _ => None,
    }
}
