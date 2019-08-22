use unic_locale_impl::canonicalize;

fn assert_canonicalize(input: &str, output: &str) {
    assert_eq!(&canonicalize(input).unwrap(), output);
}

#[test]
fn test_canonicalize_langid() {
    assert_canonicalize("Pl", "pl");
    assert_canonicalize("eN-uS", "en-US");
    assert_canonicalize("ZH_hans_hK", "zh-Hans-HK");
}

#[test]
fn test_canonicalize_locale() {
    assert_canonicalize("pl-U-HC-H12", "pl-u-hc-h12");
    assert_canonicalize("eN-uS-X_Private", "en-US-x-private");
    assert_canonicalize("eN-uS-t-pl-PL", "en-US-t-pl-PL");
}
