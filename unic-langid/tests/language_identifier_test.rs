use std::convert::TryFrom;
use unic_langid::parser::parse_language_identifier;
use unic_langid::LanguageIdentifier;

fn assert_language_identifier(
    loc: &LanguageIdentifier,
    language: Option<&str>,
    script: Option<&str>,
    region: Option<&str>,
    variants: &[&str],
) {
    assert_eq!(loc.get_language(), language.unwrap_or("und"));
    assert_eq!(loc.get_script().as_ref().map(String::as_str), script);
    assert_eq!(loc.get_region().as_ref().map(String::as_str), region);
    assert_eq!(
        loc.get_variants()
            .iter()
            .map(String::as_str)
            .collect::<Vec<&str>>(),
        variants
    );
}

fn assert_parsed_language_identifier(
    input: &str,
    language: Option<&str>,
    script: Option<&str>,
    region: Option<&str>,
    variants: &[&str],
) {
    let langid = parse_language_identifier(input).unwrap();
    assert_language_identifier(&langid, language, script, region, variants);
}

#[test]
fn test_language_identifier_parser() {
    assert_parsed_language_identifier("pl", Some("pl"), None, None, &[]);
    assert_parsed_language_identifier("en-US", Some("en"), None, Some("US"), &[]);
    assert_parsed_language_identifier("en-Latn-US", Some("en"), Some("Latn"), Some("US"), &[]);
    assert_parsed_language_identifier("sl-nedis", Some("sl"), None, None, &["nedis"]);
}

#[test]
fn test_language_casing() {
    assert_parsed_language_identifier("Pl", Some("pl"), None, None, &[]);
    assert_parsed_language_identifier("En-uS", Some("en"), None, Some("US"), &[]);
    assert_parsed_language_identifier("eN-lAtN-uS", Some("en"), Some("Latn"), Some("US"), &[]);
    assert_parsed_language_identifier("ZH_cyrl_hN", Some("zh"), Some("Cyrl"), Some("HN"), &[]);
}

#[test]
fn test_serialize_langid() {
    let langid = LanguageIdentifier::try_from("en-Latn-US").unwrap();
    assert_eq!(&langid.to_string(), "en-Latn-US");
}

#[test]
fn test_sorted_variants() {
    let langid = LanguageIdentifier::try_from("en-nedis-macos").unwrap();
    assert_eq!(&langid.to_string(), "en-macos-nedis");

    let langid =
        LanguageIdentifier::from_parts(Some("en"), None, None, &["nedis", "macos"]).unwrap();
    assert_eq!(&langid.to_string(), "en-macos-nedis");
}
