use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use unic_locale::parser::parse_locale;

fn locale_parser_bench(c: &mut Criterion) {
    let strings = &[
        "en-US-u-hc-h12",
        "en-GB-u-ca-gregory-hc-h12",
        "es-AR-x-private",
        "th-u-ca-buddhist",
        "de-u-co-phonebk-ka-shifted",
        "ar-u-nu-native",
        "ar-u-nu-latn",
        "ja-t-it",
        "ja-Kana-t-it",
        "und-Latn-t-und-cyrl",
    ];
    c.bench_function("locale_parser", move |b| {
        b.iter(|| {
            for s in strings {
                let _ = parse_locale(s);
            }
        })
    });
}

criterion_group!(benches, locale_parser_bench,);
criterion_main!(benches);
