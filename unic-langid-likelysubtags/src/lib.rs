mod tables;

use unic_langid::LanguageIdentifier;

pub fn add_likely_subtags(langid: &LanguageIdentifier) -> Option<LanguageIdentifier> {
    let lang = langid.get_language();
    let lang = if lang == "und" { None } else { Some(lang) };
    let script = langid.get_script();
    let region = langid.get_region();

    if lang.is_some() && region.is_some() && script.is_some() {
        return None;
    }

    if let Some(l) = lang {
        if let Some(r) = region {
            let result = tables::LANG_REGION
                .iter()
                .find(|(key_l, key_r, _, _, _)| key_l == &l && key_r == &r);
            if let Some(r) = result {
                return Some(LanguageIdentifier::from_parts(r.2, r.3, r.4, &[]).unwrap());
            }
        }

        if let Some(s) = script {
            let result = tables::LANG_SCRIPT
                .iter()
                .find(|(key_l, key_s, _, _, _)| key_l == &l && key_s == &s);
            if let Some(r) = result {
                return Some(LanguageIdentifier::from_parts(r.2, r.3, r.4, &[]).unwrap());
            }
        }

        let result = tables::LANG_ONLY
            .iter()
            .find(|(key_l, _, _, _)| key_l == &l);
        if let Some(r) = result {
            return Some(
                LanguageIdentifier::from_parts(r.1, script.or(r.2), region.or(r.3), &[]).unwrap(),
            );
        }
    } else if let Some(s) = script {
        if let Some(r) = region {
            let result = tables::SCRIPT_REGION
                .iter()
                .find(|(key_s, key_r, _, _, _)| key_s == &s && key_r == &r);
            if let Some(r) = result {
                return Some(LanguageIdentifier::from_parts(r.2, r.3, r.4, &[]).unwrap());
            }
        }

        let result = tables::SCRIPT_ONLY
            .iter()
            .find(|(key_s, _, _, _)| key_s == &s);
        if let Some(r) = result {
            return Some(LanguageIdentifier::from_parts(r.1, r.2, region.or(r.3), &[]).unwrap());
        }
    } else if let Some(r) = region {
        let result = tables::REGION_ONLY
            .iter()
            .find(|(key_r, _, _, _)| key_r == &r);
        if let Some(r) = result {
            return Some(LanguageIdentifier::from_parts(r.1, r.2, r.3, &[]).unwrap());
        }
    }

    None
}
