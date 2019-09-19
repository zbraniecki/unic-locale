pub mod tables;

use tinystr::{TinyStr4, TinyStr8};
use unic_langid::LanguageIdentifier;

fn get_lang_from_parts(
    input: (Option<u64>, Option<u32>, Option<u32>),
    lang: Option<u64>,
    script: Option<u32>,
    region: Option<u32>,
) -> Option<LanguageIdentifier> {
    let lang = unsafe { lang.or(input.0).map(|l| TinyStr8::new_unchecked(l)) };
    let script = unsafe { script.or(input.1).map(|s| TinyStr4::new_unchecked(s)) };
    let region = unsafe { region.or(input.2).map(|r| TinyStr4::new_unchecked(r)) };
    return Some(unsafe {
        LanguageIdentifier::from_raw_parts_unchecked(lang, script, region, None)
    });
}

pub fn add_likely_subtags(langid: &LanguageIdentifier) -> Option<LanguageIdentifier> {
    let (lang, script, region, _variants) = langid.clone().into_raw_parts();

    if lang.is_some() && region.is_some() && script.is_some() {
        return None;
    }

    if let Some(l) = lang {
        if let Some(r) = region {
            let result = tables::LANG_REGION
                .binary_search_by(|(key_l, key_r, _)| key_l.cmp(&l).then(key_r.cmp(&r)))
                .ok();
            if let Some(r) = result {
                return get_lang_from_parts(tables::LANG_REGION[r].2, None, None, None);
            }
        }

        if let Some(s) = script {
            let result = tables::LANG_SCRIPT
                .binary_search_by(|(key_l, key_s, _)| key_l.cmp(&l).then(key_s.cmp(&s)))
                .ok();
            if let Some(r) = result {
                return get_lang_from_parts(tables::LANG_SCRIPT[r].2, None, None, None);
            }
        }

        let result = tables::LANG_ONLY
            .binary_search_by(|(key_l, _)| key_l.cmp(&l))
            .ok();
        if let Some(r) = result {
            return get_lang_from_parts(tables::LANG_ONLY[r].1, None, script, region);
        }
    } else if let Some(s) = script {
        if let Some(r) = region {
            let result = tables::SCRIPT_REGION
                .binary_search_by(|(key_s, key_r, _)| key_s.cmp(&s).then(key_r.cmp(&r)))
                .ok();
            if let Some(r) = result {
                return get_lang_from_parts(tables::SCRIPT_REGION[r].2, None, None, None);
            }
        }

        let result = tables::SCRIPT_ONLY
            .binary_search_by(|(key_s, _)| key_s.cmp(&s))
            .ok();
        if let Some(r) = result {
            return get_lang_from_parts(tables::SCRIPT_ONLY[r].1, None, None, region);
        }
    } else if let Some(r) = region {
        let result = tables::REGION_ONLY
            .binary_search_by(|(key_r, _)| key_r.cmp(&r))
            .ok();
        if let Some(r) = result {
            return get_lang_from_parts(tables::REGION_ONLY[r].1, None, None, None);
        }
    }

    None
}
