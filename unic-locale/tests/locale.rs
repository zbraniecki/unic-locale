use unic_locale::Locale;
#[cfg(feature = "unic-locale-macros")]
use unic_locale::{locale, locales};

#[test]
fn basic_test() {
    let loc: Locale = "en-US".parse().expect("Malformed Locale Identifier");
    assert_eq!(&loc.to_string(), "en-US");
}

#[test]
#[cfg(feature = "unic-locale-macros")]
fn locale_macro_test() {
    let loc = locale!("en-US");
    assert_eq!(&loc.to_string(), "en-US");
}

#[test]
#[cfg(feature = "unic-locale-macros")]
fn locales_macro_test() {
    let locales = locales!["en-US-u-ca-buddhist", "pl", "de-AT-u-hc-h12", "Pl-Latn-PL"];
    assert_eq!(locales.len(), 4);
    assert_eq!(locales.get(3).unwrap().id.language.as_str(), "pl");
    assert_eq!(
        locales
            .get(0)
            .unwrap()
            .extensions
            .unicode
            .keyword("ca")
            .unwrap()
            .collect::<Vec<_>>(),
        &["buddhist"]
    );

    // check trailing comma
    locales!["en-US-u-ca-buddhist", "pl",];
}
