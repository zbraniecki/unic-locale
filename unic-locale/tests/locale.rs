use unic_locale::Locale;
#[cfg(feature = "unic-locale-macros")]
use unic_locale_macros::locale;

#[test]
fn basic_test() {
    let loc: Locale = "en-US".parse().expect("Malformed Locale Identifier");
    assert_eq!(&loc.to_string(), "en-US");
}

#[test]
#[cfg(feature = "unic-locale-macros")]
fn macro_test() {
    let loc = locale!("en-US");
    assert_eq!(&loc.to_string(), "en-US");
}
