use std::collections::HashMap;
use std::convert::TryInto;
use std::error::Error;
use std::fs::File;
use std::path::Path;

use unic_langid_impl::LanguageIdentifier;
use unic_locale_impl::{ExtensionType, ExtensionsMap, Locale};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct LocaleTestInputData {
    string: String,
    extensions: Option<HashMap<String, HashMap<String, String>>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct LocaleTestOutputObject {
    language: Option<String>,
    script: Option<String>,
    region: Option<String>,
    #[serde(default)]
    variants: Vec<String>,
    extensions: Option<HashMap<String, HashMap<String, String>>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum LocaleTestOutput {
    String(String),
    Object(LocaleTestOutputObject),
}

#[derive(Serialize, Deserialize)]
struct LocaleTestSet {
    input: LocaleTestInputData,
    output: LocaleTestOutput,
}

fn read_locale_testsets<P: AsRef<Path>>(path: P) -> Result<Vec<LocaleTestSet>, Box<dyn Error>> {
    let file = File::open(path)?;
    let sets = serde_json::from_reader(file)?;
    Ok(sets)
}

fn create_extensions_map(map: HashMap<String, HashMap<String, String>>) -> ExtensionsMap {
    let mut result = ExtensionsMap::default();
    for (key, map) in map {
        let t: ExtensionType = ExtensionType::from_byte(key.chars().nth(0).unwrap() as u8)
            .expect("Failed to format extension type.");
        match t {
            ExtensionType::Unicode => {
                for (key, value) in map {
                    result
                        .unicode
                        .set_keyword(&key, &[&value])
                        .expect("Setting extension value failed.");
                }
            }
            ExtensionType::Transform => {
                if let Some(tfield) = map.get("tlang") {
                    let tlang: LanguageIdentifier =
                        tfield.parse().expect("Parsing language identifier failed.");
                    result
                        .transform
                        .set_tlang(tlang)
                        .expect("Setting extension value failed.");
                }
            }
            ExtensionType::Private => {
                for (key, _) in map {
                    result
                        .private
                        .add_tag(&key)
                        .expect("Setting extension value failed.");
                }
            }
            _ => unimplemented!(),
        }
    }
    result
}

fn test_locale_fixtures(path: &str) {
    let tests = read_locale_testsets(path).unwrap();

    for test in tests {
        let s = test.input.string;

        let mut locale: Locale = s.parse().expect("Parsing failed.");

        if let Some(extensions) = test.input.extensions {
            for (_, map) in extensions {
                for (key, value) in map {
                    locale
                        .extensions
                        .unicode
                        .set_keyword(&key, &[&value])
                        .expect("Failed to set extension value.");
                }
            }
        }

        match test.output {
            LocaleTestOutput::Object(o) => {
                let expected = Locale::from_parts(
                    o.language.try_into().unwrap(),
                    o.script.as_ref().map(|s| s.parse().unwrap()),
                    o.region.as_ref().map(|r| r.parse().unwrap()),
                    o.variants
                        .iter()
                        .map(|s| s.parse().unwrap())
                        .collect::<Vec<_>>()
                        .as_ref(),
                    o.extensions.map(create_extensions_map),
                );
                assert_eq!(locale, expected);
            }
            LocaleTestOutput::String(s) => {
                assert_eq!(locale.to_string(), s);
            }
        }
    }
}

#[test]
fn parse() {
    test_locale_fixtures("./tests/fixtures/parsing.json");
}

#[test]
fn serialize() {
    test_locale_fixtures("./tests/fixtures/serialize.json");
}
