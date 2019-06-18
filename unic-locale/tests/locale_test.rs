use std::convert::TryFrom;

use unic_langid::LanguageIdentifier;
use unic_locale::extensions::{ExtensionsMap, UnicodeExtensionKey};
use unic_locale::parser::parse_locale;
use unic_locale::Locale;

fn assert_locale_extensions(loc: &Locale, extensions: &ExtensionsMap) {
    assert_eq!(&loc.extensions, extensions);
}

fn assert_parsed_locale_identifier(input: &str, extensions: &ExtensionsMap) {
    let loc = parse_locale(input).unwrap();
    assert_locale_extensions(&loc, extensions);
}

#[test]
fn test_basic() {
    let loc = Locale::try_from("en-US").unwrap();
    let loc2 = Locale {
        langid: LanguageIdentifier::from_parts(Some("en"), None, Some("US"), &[]).unwrap(),
        extensions: ExtensionsMap::default(),
    };
    assert_eq!(loc, loc2);
}

#[test]
fn test_from_parts() {
    let extensions = ExtensionsMap::default();
    let loc = Locale::from_parts(Some("en"), None, None, &[], Some(extensions)).unwrap();
    let loc2 = Locale {
        langid: LanguageIdentifier::from_parts(Some("en"), None, None, &[]).unwrap(),
        extensions: ExtensionsMap::default(),
    };
    assert_eq!(loc, loc2);
}

#[test]
fn test_locale_identifier() {
    let mut extensions = ExtensionsMap::default();
    extensions.set_unicode_value(UnicodeExtensionKey::HourCycle, Some("h12")).unwrap();
    assert_parsed_locale_identifier("pl-u-hc-h12", &extensions);

    let mut extensions = ExtensionsMap::default();
    extensions.set_private_value("testing", None).unwrap();
    assert_parsed_locale_identifier("und-x-testing", &extensions);
}

#[test]
fn test_serialize_locale() {
    let loc = Locale::try_from("en-u-hc-h12").unwrap();
    assert_eq!(&loc.to_string(), "en-u-hc-h12");
}

#[test]
fn test_from_langid() {
    let langid = LanguageIdentifier::from_str("en-US").unwrap();
    let loc = Locale::from(langid);
    assert_eq!(&loc.to_string(), "en-US");
}

#[test]
fn test_to_langid() {
    let loc = Locale::try_from("en-US-u-hc-h12").unwrap();
    let langid: LanguageIdentifier = loc.into();
    assert_eq!(langid.to_string(), "en-US");
}
