use unic_langid_impl::LanguageIdentifier;
use unic_langid_macros::langid;

// Currently, the const assignment will work only if
// the langid doesn't contain any variants.
const PL_PL: LanguageIdentifier = langid!("pl-PL");

fn main() {
    println!("{:#?}", PL_PL);
    let id = langid!("de-Latn-DE");
    println!("{:?}", id);
}
