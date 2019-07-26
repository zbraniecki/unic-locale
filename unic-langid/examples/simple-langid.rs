use unic_langid::LanguageIdentifier;
use unic_langid_macros::langid;

fn main() {
    let langid: LanguageIdentifier = "en-US".parse().unwrap();
    assert_eq!(langid.get_language(), "en");

    let langid = langid!("xx-US");
    assert_eq!(langid.get_language(), "en");
}