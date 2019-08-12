use unic_langid_impl::LanguageIdentifier;

fn assert_canonicalize(input: &str, output: &str) {
    assert_eq!(LanguageIdentifier::canonicalize_str(input).unwrap(), output);
}

fn assert_is_well_formed(input: &str, output: bool) {
    assert_eq!(LanguageIdentifier::is_str_well_formed(input), output);
}

#[test]
fn test_canonicalize() {
    assert_canonicalize("Pl", "pl");
    assert_canonicalize("eN-uS", "en-US");
    assert_canonicalize("ZH_hans_hK", "zh-Hans-HK");
}

#[test]
fn is_well_formed() {
    assert_is_well_formed("Pl", true);
    assert_is_well_formed("und", true);
    assert_is_well_formed("DE-latn", true);
}
