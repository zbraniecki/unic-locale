use unic_langid_impl::subtags::{Language, Region, Script, Variant};
use unic_langid_impl::LanguageIdentifier;
use unic_langid_macros::{lang, langid, region, script, variant};

// Currently, the const assignment will work only if
// the langid doesn't contain any variants.
const PL_PL: LanguageIdentifier = langid!("pl-PL");

const _: Language = lang!("pl");
const _: Script = script!("Latn");
const _: Region = region!("US");
const _: Variant = variant!("macos");

fn main() {
    println!("{:#?}", PL_PL);

    let id = langid!("de-Latn-DE");
    println!("{:#?}", id);
}
