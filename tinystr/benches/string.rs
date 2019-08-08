use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use criterion::Fun;

static STRINGS: &[&str] = &[
    "US", "GB", "AR", "Hans", "CN", "AT", "PL", "FR", "AT", "Cyrl", "SR", "NO", "FR", "MK", "UK",
];

fn string_bench_construct(c: &mut Criterion) {
    let strings = STRINGS;

    let funcs = vec![Fun::new("from_str", |b, strings: &Vec<&str>| {
        b.iter(|| {
            for s in strings {
                let _: Result<String, _> = black_box(s).parse();
            }
        })
    })];

    c.bench_functions("String_construct", funcs, strings.to_vec());
}

fn string_bench_convert(c: &mut Criterion) {
    let strings: Vec<String> = STRINGS.iter().map(|s| s.parse().unwrap()).collect();

    let funcs = vec![
        Fun::new("as_str", |b, strings: &Vec<String>| {
            b.iter(|| {
                for s in strings {
                    let _ = black_box(s).as_str();
                }
            })
        }),
        Fun::new("to_ascii_uppercase", |b, strings: &Vec<String>| {
            b.iter(|| {
                for s in strings {
                    let _ = black_box(s).to_ascii_uppercase();
                }
            })
        }),
        Fun::new("to_ascii_lowercase", |b, strings: &Vec<String>| {
            b.iter(|| {
                for s in strings {
                    let _ = black_box(s).to_ascii_lowercase();
                }
            })
        }),
        Fun::new("to_ascii_titlecase", |b, strings: &Vec<String>| {
            b.iter(|| {
                for s in strings {
                    let mut result = s.to_ascii_lowercase();
                    result[0..1].make_ascii_uppercase();
                }
            })
        }),
    ];

    c.bench_functions("String_convert", funcs, strings);
}

fn string_bench_test(c: &mut Criterion) {
    let strings: Vec<String> = STRINGS.iter().map(|s| s.parse().unwrap()).collect();

    let funcs = vec![Fun::new(
        "is_all_ascii_alphanumeric",
        |b, strings: &Vec<String>| {
            b.iter(|| {
                for s in strings {
                    let _ = black_box(s).chars().all(|c| c.is_ascii_alphanumeric());
                }
            })
        },
    )];

    c.bench_functions("String_test", funcs, strings);
}

criterion_group!(
    benches,
    string_bench_construct,
    string_bench_convert,
    string_bench_test,
);
criterion_main!(benches);
