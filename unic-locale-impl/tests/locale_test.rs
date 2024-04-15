use unic_langid_impl::LanguageIdentifier;
use unic_locale_impl::parser::parse_locale;
use unic_locale_impl::{CharacterDirection, ExtensionsMap, Locale};

fn assert_locale_extensions(loc: &Locale, extensions: &ExtensionsMap) {
    assert_eq!(&loc.extensions, extensions);
}

fn assert_parsed_locale_identifier(input: &str, extensions: &ExtensionsMap) {
    let loc = parse_locale(input).unwrap();
    assert_locale_extensions(&loc, extensions);
}

#[test]
fn test_basic() {
    let loc: Locale = "en-US".parse().unwrap();
    let loc2 = Locale {
        id: LanguageIdentifier::from_parts(
            "en".parse().unwrap(),
            None,
            Some("US".parse().unwrap()),
            &[],
        ),
        extensions: ExtensionsMap::default(),
    };
    assert_eq!(loc, loc2);
}

#[test]
fn test_from_parts() {
    let extensions = ExtensionsMap::default();
    let loc = Locale::from_parts("en".parse().unwrap(), None, None, &[], Some(extensions));
    let loc2 = Locale {
        id: LanguageIdentifier::from_parts("en".parse().unwrap(), None, None, &[]),
        extensions: ExtensionsMap::default(),
    };
    assert_eq!(loc, loc2);
}

#[test]
fn test_locale_identifier() {
    let mut extensions = ExtensionsMap::default();
    extensions.unicode.set_keyword("hc", &["h12"]).unwrap();
    assert_parsed_locale_identifier("pl-u-hc-h12", &extensions);

    extensions.unicode.set_attribute("foo").unwrap();
    assert_parsed_locale_identifier("pl-u-foo-hc-h12", &extensions);

    let val = extensions
        .unicode
        .keyword("hc")
        .unwrap()
        .collect::<Vec<_>>();
    assert_eq!(val, &["h12"]);

    let val = extensions
        .unicode
        .keyword("aa")
        .unwrap()
        .collect::<Vec<_>>();
    assert_eq!(val.is_empty(), true);

    let val = extensions.unicode.remove_keyword("hc").unwrap();
    assert_eq!(val, true);
    assert_parsed_locale_identifier("pl-u-foo", &extensions);

    let val = extensions.unicode.has_attribute("foo").unwrap();
    assert_eq!(val, true);

    let val = extensions.unicode.has_attribute("aaa").unwrap();
    assert_eq!(val, false);

    let val = extensions.unicode.remove_attribute("foo").unwrap();
    assert_eq!(val, true);
    assert_parsed_locale_identifier("pl", &extensions);

    extensions.transform.set_tfield("m0", &["foo"]).unwrap();
    assert_parsed_locale_identifier("pl-t-m0-foo", &extensions);

    let val = extensions
        .transform
        .tfield("m0")
        .unwrap()
        .collect::<Vec<_>>();
    assert_eq!(val, &["foo"]);

    let val = extensions
        .transform
        .tfield("x0")
        .unwrap()
        .collect::<Vec<_>>();
    assert_eq!(val.is_empty(), true);

    let val = extensions.transform.remove_tfield("m0").unwrap();
    assert_eq!(val, true);
    assert_parsed_locale_identifier("pl", &extensions);

    let mut extensions = ExtensionsMap::default();
    extensions.private.add_tag("testing").unwrap();
    assert_parsed_locale_identifier("und-x-testing", &extensions);
}

#[test]
fn test_serialize_locale() {
    let loc: Locale = "en-u-hc-h12".parse().unwrap();
    assert_eq!(&loc.to_string(), "en-u-hc-h12");
}

#[test]
fn test_from_langid() {
    let langid: LanguageIdentifier = "en-US".parse().unwrap();
    let loc = Locale::from(langid);
    assert_eq!(&loc.to_string(), "en-US");
}

#[test]
fn test_to_langid() {
    let loc: Locale = "en-US-u-hc-h12".parse().unwrap();
    let langid: LanguageIdentifier = loc.into();
    assert_eq!(langid.to_string(), "en-US");
}

// #[test]
// fn test_from_parts_unchecked() {
//     let loc: Locale = "en-US".parse().unwrap();
//     let (lang, script, region, variants, extensions) = loc.into_parts();
//     let loc = Locale::from_raw_parts_unchecked(
//         lang.map(|l| unsafe { TinyStr8::new_unchecked(l) }),
//         script.map(|s| unsafe { TinyStr4::new_unchecked(s) }),
//         region.map(|r| unsafe { TinyStr4::new_unchecked(r) }),
//         variants.map(|v| {
//             v.into_iter()
//                 .map(|v| unsafe { TinyStr8::new_unchecked(*v) })
//                 .collect()
//         }),
//         extensions.parse().unwrap(),
//     );
//     assert_eq!(&loc.to_string(), "en-US");
// }

#[test]
fn test_matches() {
    let loc_en: Locale = "en-u-hc-h12".parse().unwrap();
    let loc_en_us: Locale = "en-US".parse().unwrap();
    let loc_en_us2: Locale = "en-US-u-hc-h24".parse().unwrap();
    let loc_pl: Locale = "pl".parse().unwrap();
    assert_eq!(loc_en.matches(&loc_en_us, false, false), false);
    assert_eq!(loc_en_us.matches(&loc_en_us2, false, false), true);
    assert_eq!(loc_en.matches(&loc_pl, false, false), false);
    assert_eq!(loc_en.matches(&loc_en_us, true, false), true);

    let langid_en: LanguageIdentifier = "en-US".parse().unwrap();
    assert_eq!(langid_en.matches(&loc_en_us, true, true), true);
    assert_eq!(
        loc_en_us.matches(&Locale::from(langid_en), true, true),
        true
    );
}

#[test]
fn test_set_fields() {
    let mut loc = Locale::default();
    assert_eq!(&loc.to_string(), "und");

    loc.id.language = "pl".parse().expect("Setting language failed");
    assert_eq!(&loc.to_string(), "pl");

    loc.id.language = "de".parse().expect("Setting language failed");
    assert_eq!(&loc.to_string(), "de");
    loc.id.region = Some("AT".parse().expect("Setting region failed"));
    assert_eq!(&loc.to_string(), "de-AT");
    loc.id.script = Some("Latn".parse().expect("Setting script failed"));
    assert_eq!(&loc.to_string(), "de-Latn-AT");
    loc.id
        .set_variants(&["macos".parse().expect("Setting variants failed")]);
    assert_eq!(&loc.to_string(), "de-Latn-AT-macos");

    loc.id.language.clear();
    assert_eq!(&loc.to_string(), "und-Latn-AT-macos");
    loc.id.region = None;
    assert_eq!(&loc.to_string(), "und-Latn-macos");
    loc.id.script = None;
    assert_eq!(&loc.to_string(), "und-macos");
    loc.id.clear_variants();
    assert_eq!(&loc.to_string(), "und");
}

#[cfg(feature = "likelysubtags")]
#[test]
fn test_likelysubtags() {
    let mut loc_en: Locale = "en-u-hc-h12".parse().unwrap();
    assert_eq!(loc_en.id.maximize(), true);
    assert_eq!(loc_en.to_string(), "en-Latn-US-u-hc-h12");

    let mut loc_sr: Locale = "sr-Cyrl-u-hc-h12".parse().unwrap();
    assert_eq!(loc_sr.id.maximize(), true);
    assert_eq!(loc_sr.to_string(), "sr-Cyrl-RS-u-hc-h12");

    let mut loc_zh_hans: Locale = "zh-Hans-u-hc-h12".parse().unwrap();
    assert_eq!(loc_zh_hans.id.minimize(), true);
    assert_eq!(loc_zh_hans.to_string(), "zh-u-hc-h12");

    let mut loc_zh_hant: Locale = "zh-Hant-u-hc-h12".parse().unwrap();
    assert_eq!(loc_zh_hant.id.minimize(), true);
    assert_eq!(loc_zh_hant.to_string(), "zh-TW-u-hc-h12");
}

#[test]
fn test_character_direction() {
    let loc_en: Locale = "en-u-hc-h12".parse().unwrap();
    assert_eq!(loc_en.id.character_direction(), CharacterDirection::LTR);

    let loc_ar: Locale = "ar-AF-u-hc-h12".parse().unwrap();
    assert_eq!(loc_ar.id.character_direction(), CharacterDirection::RTL);

    let loc_mn: Locale = "mn-Mong".parse().unwrap();
    assert_eq!(loc_mn.id.character_direction(), CharacterDirection::TTB);

    let loc_pa_guru: Locale = "pa-Guru".parse().unwrap();
    assert_eq!(loc_pa_guru.id.character_direction(), CharacterDirection::LTR);

    let loc_pa: Locale = "pa".parse().unwrap();
    assert_eq!(loc_pa.id.character_direction(), CharacterDirection::RTL);
}

#[cfg(feature = "likelysubtags")]
#[test]
fn test_character_direction_with_likelysubtags() {
    let loc_pa: Locale = "pa".parse().unwrap();
    assert_eq!(loc_pa.id.character_direction(), CharacterDirection::LTR);

    let loc_pa_pk: Locale = "pa-PK".parse().unwrap();
    assert_eq!(loc_pa_pk.id.character_direction(), CharacterDirection::RTL);

    let loc_ar_us: Locale = "ar-US".parse().unwrap();
    assert_eq!(loc_ar_us.id.character_direction(), CharacterDirection::RTL);
}

#[test]
fn test_unicode_attributes_ordering() {
    let mut loc: Locale = "en-u-foo-bar".parse().unwrap();
    assert_eq!(&loc.to_string(), "en-u-bar-foo");

    loc.extensions
        .unicode
        .set_attribute("baz")
        .expect("Can't set attribute");
    assert_eq!(&loc.to_string(), "en-u-bar-baz-foo");
}
