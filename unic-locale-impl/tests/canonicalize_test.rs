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
    assert_canonicalize(
        "en-US-t-ES-ar-u-CA-Buddhist-x-foo",
        "en-US-t-es-AR-u-ca-buddhist-x-foo",
    );
    assert_canonicalize(
        "fr-t-t0-windows-h0-hybrid-k0-googlevk-extended",
        "fr-t-h0-hybrid-k0-googlevk-extended-t0-windows",
    );
    assert_canonicalize(
        "en-u-foo-bar-nu-thai-ca-buddhist-kk-true",
        "en-u-bar-foo-ca-buddhist-kk-nu-thai",
    );
    assert_canonicalize("en-US-u-foo-t-es-AR-x-bar", "en-US-t-es-AR-u-foo-x-bar");
    assert_canonicalize("de-u-kn-true", "de-u-kn");
    assert_canonicalize("fr-t-t0-windows-True", "fr-t-t0-windows");
}
