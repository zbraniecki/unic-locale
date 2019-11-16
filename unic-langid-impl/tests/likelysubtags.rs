use unic_langid_impl::LanguageIdentifier;

static STRINGS: &[(&str, Option<&str>)] = &[
    ("en-US", Some("en-Latn-US")),
    ("en-GB", Some("en-Latn-GB")),
    ("es-AR", Some("es-Latn-AR")),
    ("it", Some("it-Latn-IT")),
    ("zh-Hans-CN", None),
    ("de-AT", Some("de-Latn-AT")),
    ("pl", Some("pl-Latn-PL")),
    ("fr-FR", Some("fr-Latn-FR")),
    ("de-AT", Some("de-Latn-AT")),
    ("sr-Cyrl-SR", None),
    ("nb-NO", Some("nb-Latn-NO")),
    ("fr-FR", Some("fr-Latn-FR")),
    ("mk", Some("mk-Cyrl-MK")),
    ("uk", Some("uk-Cyrl-UA")),
    ("und-PL", Some("pl-Latn-PL")),
    ("und-Latn-AM", Some("ku-Latn-AM")),
    ("ug-Cyrl", Some("ug-Cyrl-KZ")),
    ("sr-ME", Some("sr-Latn-ME")),
    ("mn-Mong", Some("mn-Mong-CN")),
    ("lif-Limb", Some("lif-Limb-IN")),
    ("gan", Some("gan-Hans-CN")),
    ("zh-Hant", Some("zh-Hant-TW")),
    ("yue-Hans", Some("yue-Hans-CN")),
    ("unr", Some("unr-Beng-IN")),
    ("unr-Deva", Some("unr-Deva-NP")),
    ("und-Thai-CN", Some("lcp-Thai-CN")),
    ("ug-Cyrl", Some("ug-Cyrl-KZ")),
    ("en-Latn-DE", None),
    ("pl-FR", Some("pl-Latn-FR")),
    ("de-CH", Some("de-Latn-CH")),
    ("tuq", Some("tuq-Latn")),
    ("sr-ME", Some("sr-Latn-ME")),
    ("ng", Some("ng-Latn-NA")),
    ("klx", Some("klx-Latn")),
    ("kk-Arab", Some("kk-Arab-CN")),
    ("en-Cyrl", Some("en-Cyrl-US")),
    ("und-Cyrl-UK", Some("ru-Cyrl-UK")),
    ("und-Arab", Some("ar-Arab-EG")),
    ("und-Arab-FO", Some("ar-Arab-FO")),
    ("zh-TW", Some("zh-Hant-TW")),
];

#[cfg(any(feature = "likelysubtags-inline", feature = "likelysubtags-cldr"))]
#[test]
fn add_likely_subtags_test() {
    for i in STRINGS {
        let mut langid: LanguageIdentifier = i.0.parse().expect("Parsing failed");
        langid.add_likely_subtags();

        assert_eq!(&langid.to_string(), i.1.unwrap_or(i.0));
    }
}

#[cfg(any(feature = "likelysubtags-inline"))]
#[test]
fn version_works() {
    assert_eq!(unic_langid_impl::data::CLDR_VERSION, "36");
}

#[cfg(any(feature = "likelysubtags-inline", feature = "likelysubtags-cldr"))]
#[test]
fn remove_likely_subtags_test() {
    let mut langid: LanguageIdentifier = "zh-Hant".parse().expect("Parsing failed");
    langid.remove_likely_subtags();
    assert_eq!(&langid.to_string(), "zh-TW");

    let mut langid: LanguageIdentifier = "en-Latn-US".parse().expect("Parsing failed");
    langid.remove_likely_subtags();
    assert_eq!(&langid.to_string(), "en");
}
