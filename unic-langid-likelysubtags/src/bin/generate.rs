use serde_json::Value;
use std::fs;
use unic_langid::LanguageIdentifier;

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

        let lang = match key_langid.get_language() {
            "und" => None,
            l @ _ => Some(l),
        };
        let script = key_langid.get_script();
        let region = key_langid.get_region();

        match (lang, script, region) {
            (None, None, None) => lang_only.push(("und", value_langid)),
            (Some(_), None, None) => lang_only.push((k, value_langid)),
            (Some(_), None, Some(_)) => lang_region.push((k, value_langid)),
            (Some(_), Some(_), None) => lang_script.push((k, value_langid)),
            (None, Some(_), Some(_)) => script_region.push((k, value_langid)),
            (None, None, Some(_)) => region_only.push((k, value_langid)),
            (None, Some(_), None) => script_only.push((k, value_langid)),
            _ => {
                println!("{:#?}", key_langid);
                panic!()
            }
        }
    }

    println!("pub const LANG_ONLY: &[(&str, &str)] = &[");
    for (k, v) in lang_only {
        println!("   (\"{}\", \"{}\"),", k, &v.to_string());
    }
    println!("];");

    println!("pub const LANG_REGION: &[(&str, &str)] = &[");
    for (k, v) in lang_region {
        println!("   (\"{}\", \"{}\"),", k, &v.to_string());
    }
    println!("];");
    println!("pub const LANG_SCRIPT: &[(&str, &str)] = &[");
    for (k, v) in lang_script {
        println!("   (\"{}\", \"{}\"),", k, &v.to_string());
    }
    println!("];");
    println!("pub const SCRIPT_REGION: &[(&str, &str)] = &[");
    for (k, v) in script_region {
        println!("   (\"{}\", \"{}\"),", k, &v.to_string());
    }
    println!("];");
    println!("pub const REGION_ONLY: &[(&str, &str)] = &[");
    for (k, v) in region_only {
        println!("   (\"{}\", \"{}\"),", k, &v.to_string());
    }
    println!("];");
    println!("pub const SCRIPT_ONLY: &[(&str, &str)] = &[");
    for (k, v) in script_only {
        println!("   (\"{}\", \"{}\"),", k, &v.to_string());
    }
    println!("];");
}
