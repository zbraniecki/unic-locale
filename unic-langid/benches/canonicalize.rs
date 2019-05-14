use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use unic_langid::canonicalize;

fn langid_canonicalize_bench(c: &mut Criterion) {
    let strings = &[
        "en-us",
        "en_gb",
        "ES-AR",
        "tH",
        "DE",
        "ZH_cyrl_hN",
        "eN-lAtN-uS",
    ];
    c.bench_function("langid_canonicalize", move |b| {
        b.iter(|| {
            for s in strings {
                let _ = canonicalize(s);
            }
        })
    });
}

criterion_group!(benches, langid_canonicalize_bench,);
criterion_main!(benches);
