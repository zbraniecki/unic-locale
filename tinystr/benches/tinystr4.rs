use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use criterion::Fun;

use tinystr::TinyStr4;

static STRINGS: &[&str] = &[
    "US", "GB", "AR", "Hans", "CN", "AT", "PL", "FR", "AT", "Cyrl", "SR", "NO", "FR", "MK", "UK",
];

fn tinystr4_bench_construct(c: &mut Criterion) {
    let strings = STRINGS;

    let funcs = vec![
        Fun::new("from_str", |b, strings: &Vec<&str>| {
            b.iter(|| {
                for s in strings {
                    let _: Result<TinyStr4, _> = black_box(s).parse();
                }
            })
        }),
        Fun::new("new_unchecked", |b, strings: &Vec<&str>| {
            let raw: Vec<u32> = strings
                .iter()
                .map(|s| s.parse::<TinyStr4>().unwrap().into())
                .collect();
            b.iter(move || {
                for num in &raw {
                    let _ = unsafe { TinyStr4::new_unchecked(black_box(*num)) };
                }
            })
        }),
    ];

    c.bench_functions("tinystr4_construct", funcs, strings.to_vec());
}

fn tinystr4_bench_convert(c: &mut Criterion) {
    let strings: Vec<TinyStr4> = STRINGS.iter().map(|s| s.parse().unwrap()).collect();

    let funcs = vec![
        Fun::new("as_str", |b, strings: &Vec<TinyStr4>| {
            b.iter(|| {
                for s in strings {
                    let _ = black_box(s).as_str();
                }
            })
        }),
        Fun::new("to_ascii_uppercase", |b, strings: &Vec<TinyStr4>| {
            b.iter(|| {
                for s in strings {
                    let _ = black_box(s).to_ascii_uppercase();
                }
            })
        }),
        Fun::new("to_ascii_lowercase", |b, strings: &Vec<TinyStr4>| {
            b.iter(|| {
                for s in strings {
                    let _ = black_box(s).to_ascii_lowercase();
                }
            })
        }),
        Fun::new("to_ascii_titlecase", |b, strings: &Vec<TinyStr4>| {
            b.iter(|| {
                for s in strings {
                    let _ = black_box(s).to_ascii_titlecase();
                }
            })
        }),
    ];

    c.bench_functions("tinystr4_convert", funcs, strings);
}

fn tinystr4_bench_test(c: &mut Criterion) {
    let strings: Vec<TinyStr4> = STRINGS.iter().map(|s| s.parse().unwrap()).collect();

    let funcs = vec![Fun::new(
        "is_all_ascii_alphanumeric",
        |b, strings: &Vec<TinyStr4>| {
            b.iter(|| {
                for s in strings {
                    let _ = black_box(s).is_all_ascii_alphanumeric();
                }
            })
        },
    )];

    c.bench_functions("tinystr4_test", funcs, strings);
}

criterion_group!(
    benches,
    tinystr4_bench_construct,
    tinystr4_bench_convert,
    tinystr4_bench_test,
);
criterion_main!(benches);
