#[cfg(feature = "unic-langid-macros")]
use unic_langid::langid;
use unic_langid::LanguageIdentifier;

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
