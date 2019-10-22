mod tables;

pub use tables::CLDR_VERSION;

use tinystr::{TinyStr4, TinyStr8};

fn get_lang_from_parts(
    input: (Option<u64>, Option<u32>, Option<u32>),
    lang: Option<TinyStr8>,
    script: Option<TinyStr4>,
    region: Option<TinyStr4>,
) -> Option<(Option<TinyStr8>, Option<TinyStr4>, Option<TinyStr4>)> {
    let lang = lang.or_else(|| input.0.map(|l| unsafe { TinyStr8::new_unchecked(l) }));
    let script = script.or_else(|| input.1.map(|s| unsafe { TinyStr4::new_unchecked(s) }));
    let region = region.or_else(|| input.2.map(|r| unsafe { TinyStr4::new_unchecked(r) }));
    Some((lang, script, region))
}

pub fn add_likely_subtags(
    lang: Option<TinyStr8>,
    script: Option<TinyStr4>,
    region: Option<TinyStr4>,
) -> Option<(Option<TinyStr8>, Option<TinyStr4>, Option<TinyStr4>)> {
    if lang.is_some() && script.is_some() && region.is_some() {
        return None;
    }

    if let Some(l) = lang {
        if let Some(r) = region {
            let result = tables::LANG_REGION
                .binary_search_by(|(key_l, key_r, _)| {
                    key_l.cmp(&l.into()).then(key_r.cmp(&r.into()))
                })
                .ok();
            if let Some(r) = result {
                return get_lang_from_parts(tables::LANG_REGION[r].2, None, None, None);
            }
        }

        if let Some(s) = script {
            let result = tables::LANG_SCRIPT
                .binary_search_by(|(key_l, key_s, _)| {
                    key_l.cmp(&l.into()).then(key_s.cmp(&s.into()))
                })
                .ok();
            if let Some(r) = result {
                return get_lang_from_parts(tables::LANG_SCRIPT[r].2, None, None, None);
            }
        }

        let result = tables::LANG_ONLY
            .binary_search_by(|(key_l, _)| key_l.cmp(&l.into()))
            .ok();
        if let Some(r) = result {
            return get_lang_from_parts(tables::LANG_ONLY[r].1, None, script, region);
        }
    } else if let Some(s) = script {
        if let Some(r) = region {
            let result = tables::SCRIPT_REGION
                .binary_search_by(|(key_s, key_r, _)| {
                    key_s.cmp(&s.into()).then(key_r.cmp(&r.into()))
                })
                .ok();
            if let Some(r) = result {
                return get_lang_from_parts(tables::SCRIPT_REGION[r].2, None, None, None);
            }
        }

        let result = tables::SCRIPT_ONLY
            .binary_search_by(|(key_s, _)| key_s.cmp(&s.into()))
            .ok();
        if let Some(r) = result {
            return get_lang_from_parts(tables::SCRIPT_ONLY[r].1, None, None, region);
        }
    } else if let Some(r) = region {
        let result = tables::REGION_ONLY
            .binary_search_by(|(key_r, _)| key_r.cmp(&r.into()))
            .ok();
        if let Some(r) = result {
            return get_lang_from_parts(tables::REGION_ONLY[r].1, None, None, None);
        }
    }

    None
}

pub fn remove_likely_subtags(
    lang: Option<TinyStr8>,
    script: Option<TinyStr4>,
    region: Option<TinyStr4>,
) -> Option<(Option<TinyStr8>, Option<TinyStr4>, Option<TinyStr4>)> {
    // add_likely_subtags returns None when all 3 components are
    // already filled so don't call it in that case.
    let max_langid = if lang.is_some() && script.is_some() && region.is_some() {
        (lang, script, region)
    } else {
        add_likely_subtags(lang, script, region)?
    };

    if let Some(trial) = add_likely_subtags(max_langid.0, None, None) {
        if trial == max_langid {
            return Some((max_langid.0, None, None));
        }
    }

    if max_langid.2.is_some() {
        if let Some(trial) = add_likely_subtags(max_langid.0, None, max_langid.2) {
            if trial == max_langid {
                return Some((max_langid.0, None, max_langid.2));
            }
        }
    }

    if max_langid.1.is_some() {
        if let Some(trial) = add_likely_subtags(max_langid.0, max_langid.1, None) {
            if trial == max_langid {
                return Some((max_langid.0, max_langid.1, None));
            }
        }
    }
    None
}
