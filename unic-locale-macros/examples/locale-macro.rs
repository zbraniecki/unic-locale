use unic_locale_macros::locale;

fn main() {
    let id = locale!("de-Latn-DE-u-hc-h12-x-private");
    println!("{:?}", id);
}
