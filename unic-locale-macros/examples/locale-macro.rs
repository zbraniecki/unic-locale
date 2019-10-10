use unic_locale_macros::locale;

fn main() {
    let loc = locale!("de-Latn-DE");
    println!("{:#?}", loc);
}
