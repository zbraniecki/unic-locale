use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use std::convert::TryFrom;

use unic_langid::LanguageIdentifier;

fn language_identifier_from_str_bench(c: &mut Criterion) {
    let strings = &[
        "en-US",
        "en-GB",
        "es-AR",
        "it",
        "zh-Hans-CN",
        "de-AT",
        "pl",
        "fr-FR",
        "de-AT",
        "sr-Cyrl-SR",
        "nb-NO",
        "fr-FR",
        "mk",
        "uk",
    ];
    c.bench_function("language_identifier_from_str", move |b| {
        b.iter(|| {
            for s in strings {
                let _ = LanguageIdentifier::try_from(*s);
            }
        })
    });
}

fn language_identifier_from_parts_bench(c: &mut Criterion) {
    let entries: Vec<(Option<&str>, Option<&str>, Option<&str>, Option<Vec<&str>>)> = vec![
        (Some("en"), None, Some("US"), None),
        (Some("en"), None, Some("GB"), None),
        (Some("es"), None, Some("AR"), None),
        (Some("it"), None, None, None),
        (Some("zh"), Some("Hans"), Some("CN"), None),
        (Some("de"), None, Some("AT"), None),
        (Some("pl"), None, None, None),
        (Some("fr"), None, Some("FR"), None),
        (Some("de"), None, Some("AT"), None),
        (Some("sr"), Some("Cyrl"), Some("SR"), None),
        (Some("nb"), None, Some("NO"), None),
        (Some("fr"), None, Some("FR"), None),
        (Some("mk"), None, None, None),
        (Some("uk"), None, None, None),
    ];
    c.bench_function("language_identifier_from_parts", move |b| {
        b.iter(|| {
            for (language, region, script, variants) in &entries {
                let _ = LanguageIdentifier::from_parts(
                    language.as_ref(),
                    region.as_ref(),
                    script.as_ref(),
                    variants.as_ref(),
                );
            }
        })
    });
}

criterion_group!(
    benches,
    language_identifier_from_str_bench,
    language_identifier_from_parts_bench,,
);
criterion_main!(benches);
