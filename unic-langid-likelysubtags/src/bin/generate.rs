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
        let langid: LanguageIdentifier = k.parse().expect("Failed to parse a key.");

        let lang = if langid.get_language() == "und" {
            None
        } else {
            Some(langid.get_language())
        };
        let script = langid.get_script();
        let region = langid.get_region();

        match (lang, script, region) {
            (None, None, None) => lang_only.push(("und", v)),
            (Some(_), None, None) => lang_only.push((k, v)),
            (Some(_), None, Some(_)) => lang_region.push((k, v)),
            (Some(_), Some(_), None) => lang_script.push((k, v)),
            (None, Some(_), Some(_)) => script_region.push((k, v)),
            (None, None, Some(_)) => region_only.push((k, v)),
            (None, Some(_), None) => script_only.push((k, v)),
            _ => {
                println!("{:#?}", langid);
                panic!()
            }
        }
    }
    println!("pub const LANG_ONLY: &[(&str, &str)] = &[");
    for (k, v) in lang_only {
        println!("   (\"{}\", \"{}\"),", k, v.as_str().unwrap());
    }
    println!("];");

    println!("pub const LANG_REGION: &[(&str, &str)] = &[");
    for (k, v) in lang_region {
        println!("   (\"{}\", \"{}\"),", k, v.as_str().unwrap());
    }
    println!("];");
    println!("pub const LANG_SCRIPT: &[(&str, &str)] = &[");
    for (k, v) in lang_script {
        println!("   (\"{}\", \"{}\"),", k, v.as_str().unwrap());
    }
    println!("];");
    println!("pub const SCRIPT_REGION: &[(&str, &str)] = &[");
    for (k, v) in script_region {
        println!("   (\"{}\", \"{}\"),", k, v.as_str().unwrap());
    }
    println!("];");
    println!("pub const REGION_ONLY: &[(&str, &str)] = &[");
    for (k, v) in region_only {
        println!("   (\"{}\", \"{}\"),", k, v.as_str().unwrap());
    }
    println!("];");
    println!("pub const SCRIPT_ONLY: &[(&str, &str)] = &[");
    for (k, v) in script_only {
        println!("   (\"{}\", \"{}\"),", k, v.as_str().unwrap());
    }
    println!("];");
}
