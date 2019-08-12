#[cfg(feature = "unic-langid-macros")]
use unic_langid::langid;
use unic_langid::LanguageIdentifier;

// This will become possible when Box can be produced in a const fn
// static LANGID: LanguageIdentifier = langid!("en-US");

fn main() {
    let langid: LanguageIdentifier = "en-US".parse().unwrap();
    println!("{:#?}", langid);
    assert_eq!(langid.get_language(), "en");

    #[cfg(feature = "validity")]
    {
        use unic_langid::validity::IsValid;

        let mut langid: LanguageIdentifier = "en-US".parse().unwrap();
        assert_eq!(langid.is_valid(), None);
        assert_eq!(langid.validate().is_ok(), true);
        assert_eq!(langid.is_valid(), Some(true));
        println!("{:#?}", langid);
    }

    #[cfg(feature = "unic-langid-macros")]
    {
        let langid = langid!("de-AT");
        println!("{:#?}", langid);
        assert_eq!(langid.get_language(), "de");
    }
}
