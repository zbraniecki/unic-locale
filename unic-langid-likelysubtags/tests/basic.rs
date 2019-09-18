use unic_langid::LanguageIdentifier;
use unic_langid_likelysubtags::add_likely_subtags;

#[test]
fn sanity_check() {
    let en: LanguageIdentifier = "en".parse().unwrap();
    let en_us: LanguageIdentifier = "en-Latn-US".parse().unwrap();
    let result = add_likely_subtags(&en);
    assert_eq!(result, Some(en_us));

    let en: LanguageIdentifier = "en-Cyrl".parse().unwrap();
    let result = add_likely_subtags(&en);
    assert_eq!(result, None);

    let en: LanguageIdentifier = "en-US".parse().unwrap();
    let result = add_likely_subtags(&en);
    assert_eq!(result, None);

    let en: LanguageIdentifier = "und-Latn-US".parse().unwrap();
    let result = add_likely_subtags(&en);
    assert_eq!(result, None);
}
