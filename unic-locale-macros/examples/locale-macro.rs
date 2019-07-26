use unic_locale_impl::Locale;
use unic_locale_macros::locale;

fn main() {
    let loc: Locale = locale!("de-Latn-DE");
    println!("{:?}", loc);
}
