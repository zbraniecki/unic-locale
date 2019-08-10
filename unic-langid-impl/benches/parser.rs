use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use unic_langid_impl::parser::parse_language_identifier;

fn language_identifier_parser_bench(c: &mut Criterion) {
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
    c.bench_function("language_identifier_parser", move |b| {
        b.iter(|| {
            for s in strings {
                let _ = parse_language_identifier(s, false);
            }
        })
    });
}

fn language_identifier_parser_casing_bench(c: &mut Criterion) {
    let strings = &[
        "En_uS",
        "EN-GB",
        "ES-aR",
        "iT",
        "zH_HaNs_cN",
        "dE-aT",
        "Pl",
        "FR-FR",
        "de_AT",
        "sR-CyrL_sr",
        "NB-NO",
        "fr_fr",
        "Mk",
        "uK",
    ];
    c.bench_function("language_identifier_parser_casing", move |b| {
        b.iter(|| {
            for s in strings {
                let _ = parse_language_identifier(s, false);
            }
        })
    });
}

criterion_group!(
    benches,
    language_identifier_parser_bench,
    language_identifier_parser_casing_bench,
);
criterion_main!(benches);
