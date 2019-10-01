#[cfg(feature = "macros")]
use unic_locale::locale;
use unic_locale::Locale;

fn main() {
    let locale: Locale = "en-US".parse().unwrap();
    assert_eq!(locale.get_language(), "en");

    #[cfg(feature = "macros")]
    {
        let mut locale = locale!("en-US");
        locale
            .extensions
            .unicode
            .set_keyword("ca", vec!["buddhist"])
            .expect("Setting extension failed.");
        assert_eq!(locale.get_language(), "en");
        assert_eq!(&locale.to_string(), "en-US-u-ca-buddhist");
    }
}
