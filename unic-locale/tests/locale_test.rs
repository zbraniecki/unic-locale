use std::collections::HashMap;
use unic_langid::LanguageIdentifier;
use unic_locale::parser::parse_locale;
use unic_locale::{serialize_locale, Locale};

fn assert_locale_extensions(loc: &Locale, extensions: &HashMap<String, HashMap<String, String>>) {
    assert_eq!(&loc.extensions, extensions);
}

fn assert_parsed_locale_identifier(
    input: &str,
    extensions: &HashMap<String, HashMap<String, String>>,
) {
    let loc = parse_locale(input).unwrap();
    assert_locale_extensions(&loc, extensions);
}

#[test]
fn test_basic() {
    let loc = Locale::from_str("en-US").unwrap();
    let loc2 = Locale {
        langid: LanguageIdentifier::from_parts(Some("en"), None, Some("US"), &[]).unwrap(),
        extensions: HashMap::new(),
    };
    assert_eq!(loc, loc2);
}

#[test]
fn test_locale_identifier() {
    let mut extensions = HashMap::new();
    let mut unicode_ext = HashMap::new();
    unicode_ext.insert("hour-cycle".into(), "h12".into());
    extensions.insert("unicode".into(), unicode_ext);
    assert_parsed_locale_identifier("pl-u-hc-h12", &extensions);

    let mut extensions = HashMap::new();
    let mut private_ext = HashMap::new();
    private_ext.insert("testing".into(), "true".into());
    extensions.insert("private".into(), private_ext);
    assert_parsed_locale_identifier("und-x-testing", &extensions);
}

#[test]
fn test_serialize_locale() {
    let loc = Locale::from_str("en-u-hc-h12").unwrap();
    assert_eq!(serialize_locale(&loc).unwrap(), "en-u-hc-h12");
}
