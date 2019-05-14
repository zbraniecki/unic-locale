use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use unic_locale::{canonicalize};

fn locale_canonicalize_bench(c: &mut Criterion) {
    let strings = &[
        "en-US-u-hc-h12",
        "en-GB-u-ca-gregory-hc-h12",
        "es-AR-x-private",
        "th-u-ca-buddhist",
        "de-u-co-phonebk-ka-shifted",
        "AR_U-NU-native",
        "ar-u-nu-LaTN",
        "jA-T-it",
        "ja-kanA-T-IT",
        "unD-Latn-T-und-cyrl",
    ];
    c.bench_function("locale_canonicalize", move |b| {
        b.iter(|| {
            for s in strings {
                let _ = canonicalize(s);
            }
        })
    });
}

criterion_group!(
    benches,
    locale_canonicalize_bench,
);
criterion_main!(benches);
