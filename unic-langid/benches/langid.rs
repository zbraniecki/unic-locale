use std::convert::TryFrom;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

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
    let entries = vec![
        (Some("en"), None, Some("US"), vec![]),
        (Some("en"), None, Some("GB"), vec![]),
        (Some("es"), None, Some("AR"), vec![]),
        (Some("it"), None, None, vec![]),
        (Some("zh"), Some("Hans"), Some("CN"), vec![]),
        (Some("de"), None, Some("AT"), vec![]),
        (Some("pl"), None, None, vec![]),
        (Some("fr"), None, Some("FR"), vec![]),
        (Some("de"), None, Some("AT"), vec![]),
        (Some("sr"), Some("Cyrl"), Some("SR"), vec![]),
        (Some("nb"), None, Some("NO"), vec![]),
        (Some("fr"), None, Some("FR"), vec![]),
        (Some("mk"), None, None, vec![]),
        (Some("uk"), None, None, vec![]),
    ];
    c.bench_function("language_identifier_from_parts", move |b| {
        b.iter(|| {
            for (language, region, script, variants) in &entries {
                let _ = LanguageIdentifier::from_parts(*language, *region, *script, &*variants);
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
