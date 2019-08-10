#[cfg(feature = "unic-langid-macros")]
use unic_langid::langid;
use unic_langid::LanguageIdentifier;

// This will become possible when Box can be produced in a const fn
// static LANGID: LanguageIdentifier = langid!("en-US");

fn main() {
    let langid: LanguageIdentifier = "en-US".parse().unwrap();
    println!("{:#?}", langid);
    assert_eq!(langid.get_language(), "en");

    #[cfg(feature = "unic-langid-macros")]
    {
        let langid = langid!("de-AT");
        println!("{:#?}", langid);
        assert_eq!(langid.get_language(), "de");
    }
}
