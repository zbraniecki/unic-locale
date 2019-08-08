use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use criterion::Fun;

use tinystr::TinyStr8;

static STRINGS: &[&str] = &[
    "US", "GB", "AR", "Hans", "CN", "AT", "PL", "FR", "AT", "Cyrl", "SR", "NO", "FR", "MK", "UK",
];

fn tinystr8_bench_construct(c: &mut Criterion) {
    let strings = STRINGS;

    let funcs = vec![
        Fun::new("from_str", |b, strings: &Vec<&str>| {
            b.iter(|| {
                for s in strings {
                    let _: Result<TinyStr8, _> = black_box(s).parse();
                }
            })
        }),
        Fun::new("new_unchecked", |b, strings: &Vec<&str>| {
            let raw: Vec<u64> = strings
                .iter()
                .map(|s| s.parse::<TinyStr8>().unwrap().into())
                .collect();
            b.iter(move || {
                for num in &raw {
                    let _ = unsafe { TinyStr8::new_unchecked(black_box(*num)) };
                }
            })
        }),
    ];

    c.bench_functions("tinystr8_construct", funcs, strings.to_vec());
}

fn tinystr8_bench_convert(c: &mut Criterion) {
    let strings: Vec<TinyStr8> = STRINGS.iter().map(|s| s.parse().unwrap()).collect();

    let funcs = vec![
        Fun::new("as_str", |b, strings: &Vec<TinyStr8>| {
            b.iter(|| {
                for s in strings {
                    let _ = black_box(s).as_str();
                }
            })
        }),
        Fun::new("to_ascii_uppercase", |b, strings: &Vec<TinyStr8>| {
            b.iter(|| {
                for s in strings {
                    let _ = black_box(s).to_ascii_uppercase();
                }
            })
        }),
        Fun::new("to_ascii_lowercase", |b, strings: &Vec<TinyStr8>| {
            b.iter(|| {
                for s in strings {
                    let _ = black_box(s).to_ascii_lowercase();
                }
            })
        }),
    ];

    c.bench_functions("tinystr8_convert", funcs, strings);
}

fn tinystr8_bench_test(c: &mut Criterion) {
    let strings: Vec<TinyStr8> = STRINGS.iter().map(|s| s.parse().unwrap()).collect();

    let funcs = vec![Fun::new(
        "is_all_ascii_alphanumeric",
        |b, strings: &Vec<TinyStr8>| {
            b.iter(|| {
                for s in strings {
                    let _ = black_box(s).is_all_ascii_alphanumeric();
                }
            })
        },
    )];

    c.bench_functions("tinystr8_test", funcs, strings);
}

criterion_group!(
    benches,
    tinystr8_bench_construct,
    tinystr8_bench_convert,
    tinystr8_bench_test,
);
criterion_main!(benches);
