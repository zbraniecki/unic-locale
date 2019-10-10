#[cfg(feature = "macros")]
use unic_locale::locale;
use unic_locale::Locale;

fn main() {
    let mut locale: Locale = "fr-CA".parse().unwrap();
    locale
        .extensions
        .unicode
        .set_keyword("ca", vec!["buddhist"])
        .expect("Setting extension failed.");

    println!("{:#?}", locale);
    assert_eq!(locale.get_language(), "fr");
    assert_eq!(&locale.to_string(), "fr-CA-u-ca-buddhist");

    #[cfg(feature = "macros")]
    {
        let langid = locale!("de-AT");
        println!("{:#?}", langid);
        assert_eq!(langid.get_language(), "de");
    }
}
