#[cfg(feature = "macros")]
use unic_locale::locale;
use unic_locale::Locale;

fn main() {
    let locale: Locale = "en-US".parse().unwrap();
    assert_eq!(locale.get_language(), "en");

    #[cfg(feature = "macros")]
    {
        let locale = locale!("en-US");
        assert_eq!(locale.get_language(), "en");
    }
}
