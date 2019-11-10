use unic_locale_macros::locale;

fn main() {
    let loc = locale!("de-Latn-DE-u-ca-buddhist");
    println!("{:#?}", loc);
}
