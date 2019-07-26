#[cfg(feature = "macros")]
use unic_langid::langid;
use unic_langid::LanguageIdentifier;

fn main() {
    let langid: LanguageIdentifier = "en-US".parse().unwrap();
    assert_eq!(langid.get_language(), "en");

    #[cfg(feature = "macros")]
    {
        let langid = langid!("en-US");
        assert_eq!(langid.get_language(), "en");
    }
}
