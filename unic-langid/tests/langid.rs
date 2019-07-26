#[cfg(feature = "unic-langid-macros")]
use unic_langid::langid;
use unic_langid::LanguageIdentifier;

#[test]
fn basic_test() {
    let loc: LanguageIdentifier = "en-US".parse().expect("Malformed Language Identifier");
    assert_eq!(&loc.to_string(), "en-US");
}

#[test]
#[cfg(feature = "unic-langid-macros")]
fn macro_test() {
    let loc = langid!("en-US");
    assert_eq!(&loc.to_string(), "en-US");
}
