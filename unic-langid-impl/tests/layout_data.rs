#[cfg(any(feature = "layout-inline", feature = "layout-cldr"))]
#[test]
fn test_character_direction() {
    use unic_langid_impl::{CharacterDirection, LanguageIdentifier};
    let langid: LanguageIdentifier = "en-US".parse().unwrap();
    let langid2: LanguageIdentifier = "ar-AF".parse().unwrap();
    assert_eq!(langid.get_character_direction(), CharacterDirection::LTR);
    assert_eq!(langid2.get_character_direction(), CharacterDirection::RTL);
}
