use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::Path;

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

fn read_locale_testsets<P: AsRef<Path>>(path: P) -> Result<Vec<LocaleTestSet>, Box<Error>> {
    let file = File::open(path)?;
    let sets = serde_json::from_reader(file)?;
    Ok(sets)
}

fn create_extensions_map(map: HashMap<String, HashMap<String, String>>) -> ExtensionsMap {
    let mut result = ExtensionsMap::default();
    for (key, map) in map {
        let t: ExtensionType = ExtensionType::from_char(key.chars().nth(0).unwrap())
            .expect("Failed to format extension type.");
        for (key, value) in map {
            match t {
                ExtensionType::Unicode => {
                    result
                        .set_unicode_value(&key, Some(value.as_str()))
                        .expect("Setting extension value failed.");
                }
                _ => unimplemented!(),
            }
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
            for (key, map) in extensions {
                let t: ExtensionType = ExtensionType::from_char(key.chars().nth(0).unwrap())
                    .expect("Failed to format extension type.");
                for (key, value) in map {
                    locale
                        .set_extension(t, &key, Some(value.as_str()))
                        .expect("Failed to set extension value.");
                }
            }
        }

        match test.output {
            LocaleTestOutput::Object(o) => {
                let expected = Locale::from_parts(
                    o.language.as_ref().map(String::as_str),
                    o.script.as_ref().map(String::as_str),
                    o.region.as_ref().map(String::as_str),
                    o.variants
                        .iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<_>>()
                        .as_ref(),
                    o.extensions.map(create_extensions_map),
                )
                .expect("Parsing failed.");
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
