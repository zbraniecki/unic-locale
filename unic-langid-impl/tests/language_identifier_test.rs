use tinystr::{TinyStr4, TinyStr8};
use unic_langid_impl::parser::parse_language_identifier;
use unic_langid_impl::parser::errors::ParserError;
use unic_langid_impl::{LanguageIdentifier, errors::LanguageIdentifierError};

fn assert_language_identifier(
    loc: &LanguageIdentifier,
    language: Option<&str>,
    script: Option<&str>,
    region: Option<&str>,
    variants: Option<&[&str]>,
) {
    assert_eq!(loc.get_language(), language.unwrap_or("und"));
    assert_eq!(loc.get_script(), script);
    assert_eq!(loc.get_region(), region);
    assert_eq!(loc.get_variants(), variants.unwrap_or(&[]));
}

fn assert_parsed_language_identifier(
    input: &str,
    language: Option<&str>,
    script: Option<&str>,
    region: Option<&str>,
    variants: Option<&[&str]>,
) {
    let langid = parse_language_identifier(input).unwrap();
    assert_language_identifier(&langid, language, script, region, variants);
}

#[test]
fn test_language_identifier_parser() {
    assert_parsed_language_identifier("pl", Some("pl"), None, None, None);
    assert_parsed_language_identifier("und", None, None, None, None);
    assert_parsed_language_identifier("en-US", Some("en"), None, Some("US"), None);
    assert_parsed_language_identifier("en-Latn-US", Some("en"), Some("Latn"), Some("US"), None);
    assert_parsed_language_identifier("sl-nedis", Some("sl"), None, None, Some(&["nedis"]));
}

#[test]
fn test_language_casing() {
    assert_parsed_language_identifier("Pl", Some("pl"), None, None, None);
    assert_parsed_language_identifier("En-uS", Some("en"), None, Some("US"), None);
    assert_parsed_language_identifier("eN-lAtN-uS", Some("en"), Some("Latn"), Some("US"), None);
    assert_parsed_language_identifier("ZH_cyrl_hN", Some("zh"), Some("Cyrl"), Some("HN"), None);
}

#[test]
fn test_serialize_langid() {
    let langid: LanguageIdentifier = "en-Latn-US".parse().unwrap();
    assert_eq!(&langid.to_string(), "en-Latn-US");
}

#[test]
fn test_sorted_variants() {
    let langid: LanguageIdentifier = "en-nedis-macos".parse().unwrap();
    assert_eq!(&langid.to_string(), "en-macos-nedis");

    let langid =
        LanguageIdentifier::from_parts(Some("en"), None, None, &["nedis", "macos"]).unwrap();
    assert_eq!(&langid.to_string(), "en-macos-nedis");
}

#[test]
fn test_from_parts() {
    let langid =
        LanguageIdentifier::from_parts(Some("en"), None, None, &["1", "macos"]);
    assert_eq!(langid, Err(LanguageIdentifierError::ParserError(ParserError::InvalidSubtag)));
}

#[test]
fn test_from_parts_unchecked() {
    let langid: LanguageIdentifier = "en-nedis-macos".parse().unwrap();
    let (lang, script, region, variants) = langid.into_raw_parts();
    let langid = unsafe {
        LanguageIdentifier::from_raw_parts_unchecked(
            lang.map(|l| TinyStr8::new_unchecked(l)),
            script.map(|s| TinyStr4::new_unchecked(s)),
            region.map(|r| TinyStr4::new_unchecked(r)),
            variants
                .map(|v| {
                    v.into_iter()
                    .map(|v| TinyStr8::new_unchecked(*v))
                    .collect()
                }),
        )
    };
    assert_eq!(&langid.to_string(), "en-macos-nedis");
}

#[test]
fn test_matches() {
    let langid_en: LanguageIdentifier = "en".parse().unwrap();
    let langid_en_us: LanguageIdentifier = "en-US".parse().unwrap();
    let langid_en_us2: LanguageIdentifier = "en-US".parse().unwrap();
    let langid_pl: LanguageIdentifier = "pl".parse().unwrap();
    assert_eq!(langid_en.matches(&langid_en_us, false, false), false);
    assert_eq!(langid_en_us.matches(&langid_en_us2, false, false), true);
    assert_eq!(langid_en.matches(&langid_pl, false, false), false);
    assert_eq!(langid_en.matches(&langid_en_us, true, false), true);
}

#[test]
fn test_set_fields() {
    let mut langid = LanguageIdentifier::default();
    assert_eq!(&langid.to_string(), "und");

    langid
        .set_language(Some("pl"))
        .expect("Setting language failed");
    assert_eq!(&langid.to_string(), "pl");

    langid
        .set_language(Some("de"))
        .expect("Setting language failed");
    assert_eq!(&langid.to_string(), "de");
    langid
        .set_region(Some("AT"))
        .expect("Setting region failed");
    assert_eq!(&langid.to_string(), "de-AT");
    langid
        .set_script(Some("Latn"))
        .expect("Setting script failed");
    assert_eq!(&langid.to_string(), "de-Latn-AT");
    langid
        .set_variants(&["macos"])
        .expect("Setting variants failed");
    assert_eq!(&langid.to_string(), "de-Latn-AT-macos");

    langid.set_language(None).expect("Setting language failed");
    assert_eq!(&langid.to_string(), "und-Latn-AT-macos");
    langid.set_region(None).expect("Setting region failed");
    assert_eq!(&langid.to_string(), "und-Latn-macos");
    langid.set_script(None).expect("Setting script failed");
    assert_eq!(&langid.to_string(), "und-macos");
    langid.set_variants(&[]).expect("Setting variants failed");
    assert_eq!(&langid.to_string(), "und");
}
