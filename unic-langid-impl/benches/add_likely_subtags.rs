use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use criterion::black_box;

use unic_langid_impl::LanguageIdentifier;

static STRINGS: &[&str] = &[
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
    "und-PL",
    "und-Latn-AM",
    "ug-Cyrl",
    "sr-ME",
    "mn-Mong",
    "lif-Limb",
    "gan",
    "zh-Hant",
    "yue-Hans",
    "unr",
    "unr-Deva",
    "und-Thai-CN",
    "ug-Cyrl",
    "en-Latn-DE",
    "pl-FR",
    "de-CH",
    "tuq",
    "sr-ME",
    "ng",
    "klx",
    "kk-Arab",
    "en-Cyrl",
    "und-Cyrl-UK",
    "und-Arab",
    "und-Arab-FO",
];

fn add_likely_subtags_bench(c: &mut Criterion) {

    c.bench_function("add_likely_subtags", move |b| {
        b.iter(|| {
            let langids: Vec<LanguageIdentifier> = STRINGS
                .iter()
                .map(|s| -> LanguageIdentifier { s.parse().unwrap() })
                .collect();
            for mut s in langids {
                #[cfg(feature="likelysubtags")]
                s.add_likely_subtags();
                let _ = black_box(s.to_string());
            }
        })
    });
}

criterion_group!(benches, add_likely_subtags_bench,);
criterion_main!(benches);
