use unic_langid_macros::langid;
use unic_langid_impl::LanguageIdentifier;

const PL_PL: LanguageIdentifier = langid!("pl-PL");

fn main() {
    println!("{:#?}", PL_PL);
    let id = langid!("de-Latn-DE");
    println!("{:?}", id);
}
