use serde_json::Value;
use std::fs;
use unic_langid::LanguageIdentifier;

fn get_lang_option(l: &str) -> Option<&str> {
    if l == "und" {
        None
    } else {
        Some(l)
    }
}

fn serialize_lang(l: &str) -> String {
    if l != "und" {
        format!("Some(\"{}\")", l)
    } else {
        String::from("None")
    }
}

fn serialize_subtag(l: Option<String>) -> String {
    if let Some(l) = l {
        format!("Some(\"{}\")", l)
    } else {
        String::from("None")
    }
}

fn main() {
    let contents = fs::read_to_string("./data/likelySubtags.json")
        .expect("Something went wrong reading the file");
    let v: Value = serde_json::from_str(&contents).unwrap();
    let values = v["supplemental"]["likelySubtags"].as_object().unwrap();

    let mut lang_only = vec![];
    let mut lang_region = vec![];
    let mut lang_script = vec![];
    let mut script_region = vec![];
    let mut region_only = vec![];
    let mut script_only = vec![];

    for (k, v) in values {
        let key_langid: LanguageIdentifier = k.parse().expect("Failed to parse a key.");
        let v: &str = v.as_str().unwrap();
        let mut value_langid: LanguageIdentifier = v.parse().expect("Failed to parse a value.");

        if let Some("ZZ") = value_langid.get_region() {
            value_langid.set_region(None).unwrap();
        }

        let lang = key_langid.get_language();
        let script = key_langid.get_script();
        let region = key_langid.get_region();

        match (lang, script, region) {
            (l, None, None) => lang_only.push((
                l.to_string(),
                value_langid.get_language().to_string(),
                value_langid.get_script().map(|s| s.to_owned()),
                value_langid.get_region().map(|s| s.to_owned()),
            )),
            (l, None, Some(r)) if l != "und" => lang_region.push((
                l.to_string(),
                r.to_owned(),
                value_langid.get_language().to_string(),
                value_langid.get_script().map(|s| s.to_owned()),
                value_langid.get_region().map(|s| s.to_owned()),
            )),
            (l, Some(s), None) if l != "und" => lang_script.push((
                l.to_string(),
                s.to_owned(),
                value_langid.get_language().to_string(),
                value_langid.get_script().map(|s| s.to_owned()),
                value_langid.get_region().map(|s| s.to_owned()),
            )),
            ("und", Some(s), Some(r)) => script_region.push((
                s.to_string(),
                r.to_owned(),
                value_langid.get_language().to_string(),
                value_langid.get_script().map(|s| s.to_owned()),
                value_langid.get_region().map(|s| s.to_owned()),
            )),
            ("und", Some(s), None) => script_only.push((
                s.to_owned(),
                value_langid.get_language().to_string(),
                value_langid.get_script().map(|s| s.to_owned()),
                value_langid.get_region().map(|s| s.to_owned()),
            )),
            ("und", None, Some(r)) => region_only.push((
                r.to_owned(),
                value_langid.get_language().to_string(),
                value_langid.get_script().map(|s| s.to_owned()),
                value_langid.get_region().map(|s| s.to_owned()),
            )),
            _ => {
                println!("{:#?}", key_langid);
                panic!()
            }
        }
    }

    println!("pub const LANG_ONLY: &[(&str, Option<&str>, Option<&str>, Option<&str>)] = &[");
    for (key_lang, val_lang, val_script, val_region) in lang_only {
        println!(
            "   (\"{}\", {}, {}, {}),",
            key_lang,
            serialize_lang(&val_lang),
            serialize_subtag(val_script),
            serialize_subtag(val_region),
        );
    }
    println!("];");

    println!(
        "pub const LANG_REGION: &[(&str, &str, Option<&str>, Option<&str>, Option<&str>)] = &["
    );
    for (key_lang, key_region, val_lang, val_script, val_region) in lang_region {
        println!(
            "   (\"{}\", \"{}\", {}, {}, {}),",
            key_lang,
            key_region,
            serialize_lang(&val_lang),
            serialize_subtag(val_script),
            serialize_subtag(val_region),
        );
    }
    println!("];");
    println!(
        "pub const LANG_SCRIPT: &[(&str, &str, Option<&str>, Option<&str>, Option<&str>)] = &["
    );
    for (key_lang, key_script, val_lang, val_script, val_region) in lang_script {
        println!(
            "   (\"{}\", \"{}\", {}, {}, {}),",
            key_lang,
            key_script,
            serialize_lang(&val_lang),
            serialize_subtag(val_script),
            serialize_subtag(val_region),
        );
    }
    println!("];");
    println!(
        "pub const SCRIPT_REGION: &[(&str, &str, Option<&str>, Option<&str>, Option<&str>)] = &["
    );
    for (key_script, key_region, val_lang, val_script, val_region) in script_region {
        println!(
            "   (\"{}\", \"{}\", {}, {}, {}),",
            key_script,
            key_region,
            serialize_lang(&val_lang),
            serialize_subtag(val_script),
            serialize_subtag(val_region),
        );
    }
    println!("];");
    println!("pub const SCRIPT_ONLY: &[(&str, Option<&str>, Option<&str>, Option<&str>)] = &[");
    for (key_script, val_lang, val_script, val_region) in script_only {
        println!(
            "   (\"{}\", {}, {}, {}),",
            key_script,
            serialize_lang(&val_lang),
            serialize_subtag(val_script),
            serialize_subtag(val_region),
        );
    }
    println!("];");
    println!("pub const REGION_ONLY: &[(&str, Option<&str>, Option<&str>, Option<&str>)] = &[");
    for (key_region, val_lang, val_script, val_region) in region_only {
        println!(
            "   (\"{}\", {}, {}, {}),",
            key_region,
            serialize_lang(&val_lang),
            serialize_subtag(val_script),
            serialize_subtag(val_region),
        );
    }
    println!("];");
}
