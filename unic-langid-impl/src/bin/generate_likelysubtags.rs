use serde_json::Value;
use std::fs;
use std::str::FromStr;
use tinystr::{TinyStr4, TinyStr8};
use unic_langid_impl::LanguageIdentifier;

type LangIdSubTags = (Option<Option<u64>>, Option<Option<u32>>, Option<Option<u32>>);

fn serialize_val(input: LangIdSubTags) -> String {
    format!(
        "({}, {}, {})",
        serialize_lang_option(input.0),
        serialize_script_option(input.1),
        serialize_region_option(input.2)
    )
}

fn serialize_lang_option(l: Option<Option<u64>>) -> String {
    match l {
        Some(Some(l)) => format!("SubtagOverride::Some({})", l),
        Some(None) => format!("SubtagOverride::None"),
        None => format!("SubtagOverride::Same"),
    }
}

fn serialize_script_option(s: Option<Option<u32>>) -> String {
    match s {
        Some(Some(s)) => format!("SubtagOverride::Some({})", s),
        Some(None) => format!("SubtagOverride::None"),
        None => format!("SubtagOverride::Same"),
    }
}

fn serialize_region_option(r: Option<Option<u32>>) -> String {
    match r {
        Some(Some(r)) => format!("SubtagOverride::Some({})", r),
        Some(None) => format!("SubtagOverride::None"),
        None => format!("SubtagOverride::Same"),
    }
}

fn main() {
    let contents = fs::read_to_string("./data/likelySubtags.json")
        .expect("Something went wrong reading the file");
    let v: Value = serde_json::from_str(&contents).unwrap();
    let values = v["supplemental"]["likelySubtags"].as_object().unwrap();

    let mut lang_only: Vec<(u64, LangIdSubTags)> = vec![];
    let mut lang_region: Vec<(u64, u32, LangIdSubTags)> = vec![];
    let mut lang_script: Vec<(u64, u32, LangIdSubTags)> = vec![];
    let mut script_region: Vec<(u32, u32, LangIdSubTags)> = vec![];
    let mut region_only: Vec<(u32, LangIdSubTags)> = vec![];
    let mut script_only: Vec<(u32, LangIdSubTags)> = vec![];

    for (k, v) in values {
        let key_langid: LanguageIdentifier = k.parse().expect("Failed to parse a key.");
        let v: &str = v.as_str().unwrap();
        let mut value_langid: LanguageIdentifier = v.parse().expect("Failed to parse a value.");
        if let Some("ZZ") = value_langid.get_region() {
            value_langid.clear_region();
        }
        let (val_lang, val_script, val_region, _) = value_langid.into_raw_parts();

        let lang = key_langid.get_language();
        let script = key_langid.get_script();
        let region = key_langid.get_region();

        match (lang, script, region) {
            (l, None, None) => {
                let input_lang = TinyStr8::from_str(l).unwrap().into();
                let new_lang = if Some(input_lang) != val_lang {
                    Some(val_lang)
                } else {
                    None
                };
                lang_only.push((
                        input_lang,
                        (new_lang, Some(val_script), Some(val_region)),
                        ))
            },
            (l, None, Some(r)) if l != "und" => {
                let input_lang = TinyStr8::from_str(l).unwrap().into();
                let input_region = TinyStr4::from_str(r).unwrap().into();
                let new_lang = if Some(input_lang) != val_lang {
                    Some(val_lang)
                } else {
                    None
                };
                let new_region = if Some(input_region) != val_region {
                    Some(val_region)
                } else {
                    None
                };
                lang_region.push((
                TinyStr8::from_str(l).unwrap().into(),
                TinyStr4::from_str(r).unwrap().into(),
                (new_lang, Some(val_script), new_region),
            ))},
            (l, Some(s), None) if l != "und" => {
                let input_lang = TinyStr8::from_str(l).unwrap().into();
                let input_script = TinyStr4::from_str(s).unwrap().into();
                let new_lang = if Some(input_lang) != val_lang {
                    Some(val_lang)
                } else {
                    None
                };
                let new_script = if Some(input_script) != val_script {
                    Some(val_script)
                } else {
                    None
                };
                lang_script.push((
                TinyStr8::from_str(l).unwrap().into(),
                TinyStr4::from_str(s).unwrap().into(),
                (new_lang, new_script, Some(val_region)),
            ))},
            ("und", Some(s), Some(r)) => {
                let input_script = TinyStr4::from_str(s).unwrap().into();
                let input_region = TinyStr4::from_str(r).unwrap().into();
                let new_script = if Some(input_script) != val_script {
                    Some(val_script)
                } else {
                    None
                };
                let new_region = if Some(input_region) != val_region {
                    Some(val_region)
                } else {
                    None
                };
                script_region.push((
                TinyStr4::from_str(s).unwrap().into(),
                TinyStr4::from_str(r).unwrap().into(),
                (Some(val_lang), new_script, new_region),
            ))
            },
            ("und", Some(s), None) => {
                let input_script = TinyStr4::from_str(s).unwrap().into();
                let new_script = if Some(input_script) != val_script {
                    Some(val_script)
                } else {
                    None
                };
                script_only.push((
                TinyStr4::from_str(s).unwrap().into(),
                (Some(val_lang), new_script, Some(val_region)),
            ))
            },
            ("und", None, Some(r)) => {
                let input_region = TinyStr4::from_str(r).unwrap().into();
                let new_region = if Some(input_region) != val_region {
                    Some(val_region)
                } else {
                    None
                };
                region_only.push((
                TinyStr4::from_str(r).unwrap().into(),
                (Some(val_lang), Some(val_script), new_region),
            ))
            },
            _ => {
                panic!("{:#?}", key_langid);
            }
        }
    }

    println!("#![allow(clippy::type_complexity)]");
    println!("#![allow(clippy::unreadable_literal)]\n");

    let version = v["supplemental"]["version"]["_cldrVersion"]
        .as_str()
        .unwrap();
    println!("pub const CLDR_VERSION: &str = \"{}\";", version);
    println!("pub use super::SubtagOverride;");

    println!(
        "pub const LANG_ONLY: &[(u64, (SubtagOverride<u64>, SubtagOverride<u32>, SubtagOverride<u32>)); {}] = &[",
        lang_only.len()
    );
    lang_only.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    for (key_lang, val) in lang_only {
        println!("    ({}, {}),", key_lang, serialize_val(val),);
    }
    println!("];");

    println!(
        "pub const LANG_REGION: [(u64, u32, (SubtagOverride<u64>, SubtagOverride<u32>, SubtagOverride<u32>)); {}] = [",
        lang_region.len()
    );
    lang_region.sort_by(|a, b| {
        a.0.partial_cmp(&b.0)
            .unwrap()
            .then_with(|| a.1.partial_cmp(&b.1).unwrap())
    });
    for (key_lang, key_region, val) in lang_region {
        println!(
            "    ({}, {}, {}),",
            key_lang,
            key_region,
            serialize_val(val),
        );
    }
    println!("];");
    println!(
        "pub const LANG_SCRIPT: [(u64, u32, (SubtagOverride<u64>, SubtagOverride<u32>, SubtagOverride<u32>)); {}] = [",
        lang_script.len()
    );
    lang_script.sort_by(|a, b| {
        a.0.partial_cmp(&b.0)
            .unwrap()
            .then_with(|| a.1.partial_cmp(&b.1).unwrap())
    });
    for (key_lang, key_script, val) in lang_script {
        println!(
            "    ({}, {}, {}),",
            key_lang,
            key_script,
            serialize_val(val),
        );
    }
    println!("];");
    println!(
        "pub const SCRIPT_REGION: [(u32, u32, (SubtagOverride<u64>, SubtagOverride<u32>, SubtagOverride<u32>)); {}] = [",
        script_region.len()
    );
    script_region.sort_by(|a, b| {
        a.0.partial_cmp(&b.0)
            .unwrap()
            .then_with(|| a.1.partial_cmp(&b.1).unwrap())
    });
    for (key_script, key_region, val) in script_region {
        println!(
            "    ({}, {}, {}),",
            key_script,
            key_region,
            serialize_val(val),
        );
    }
    println!("];");
    println!(
        "pub const SCRIPT_ONLY: [(u32, (SubtagOverride<u64>, SubtagOverride<u32>, SubtagOverride<u32>)); {}] = [",
        script_only.len()
    );
    script_only.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    for (key_script, val) in script_only {
        println!("    ({}, {}),", key_script, serialize_val(val),);
    }
    println!("];");
    println!(
        "pub const REGION_ONLY: [(u32, (SubtagOverride<u64>, SubtagOverride<u32>, SubtagOverride<u32>)); {}] = [",
        region_only.len()
    );
    region_only.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    for (key_region, val) in region_only {
        println!("    ({}, {}),", key_region, serialize_val(val),);
    }
    println!("];");
}
