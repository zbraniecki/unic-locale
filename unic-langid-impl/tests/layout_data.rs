use unic_langid_impl::{CharacterDirection, LanguageIdentifier};

#[test]
fn test_character_direction() {
    let langid: LanguageIdentifier = "en-US".parse().unwrap();
    let langid2: LanguageIdentifier = "ar-AF".parse().unwrap();
    assert_eq!(langid.get_character_direction(), CharacterDirection::LTR);
    assert_eq!(langid2.get_character_direction(), CharacterDirection::RTL);
}
