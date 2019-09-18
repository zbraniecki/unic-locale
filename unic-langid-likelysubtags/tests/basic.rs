use unic_langid::LanguageIdentifier;
use unic_langid_likelysubtags::add_likely_subtags;

static STRINGS: &[(&str, Option<&str>)] = &[
    ("en-US", None),
    ("en-GB", None),
    ("es-AR", None),
    ("it", Some("it-Latn-IT")),
    ("zh-Hans-CN", None),
    ("de-AT", None),
    ("pl", Some("pl-Latn-PL")),
    ("fr-FR", None),
    ("de-AT", None),
    ("sr-Cyrl-SR", None),
    ("nb-NO", None),
    ("fr-FR", None),
    ("mk", Some("mk-Cyrl-MK")),
    ("uk", Some("uk-Cyrl-UA")),
    ("und-PL", None),
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
    ("pl-FR", None),
    ("de-CH", None),
    ("tuq", Some("tuq-Latn")),
    ("sr-ME", Some("sr-Latn-ME")),
    ("ng", Some("ng-Latn-NA")),
    ("klx", Some("klx-Latn")),
    ("kk-Arab", Some("kk-Arab-CN")),
];

#[test]
fn sanity_check() {
    for i in STRINGS {
        let en: LanguageIdentifier = i.0.parse().unwrap();
        let result = add_likely_subtags(&en).map(|l| l.to_string());
        assert_eq!(result.as_ref().map(String::as_str), i.1);
    }
}
