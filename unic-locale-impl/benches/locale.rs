use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use criterion::Fun;

use tinystr::{TinyStr4, TinyStr8};
use unic_locale_impl::{ExtensionsMap, Locale};

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
];

fn locale_construct_bench(c: &mut Criterion) {
    let locales: Vec<Locale> = STRINGS
        .iter()
        .map(|s| -> Locale { s.parse().unwrap() })
        .collect();

    let funcs = vec![
        Fun::new("from_str", |b, _| {
            b.iter(|| {
                for s in STRINGS {
                    let _: Result<Locale, _> = s.parse();
                }
            })
        }),
        Fun::new("from_parts", |b, locales: &Vec<Locale>| {
            let entries: Vec<(
                Option<&str>,
                Option<&str>,
                Option<&str>,
                Vec<&str>,
                Option<ExtensionsMap>,
            )> = locales
                .iter()
                .map(|locale| {
                    let lang = Some(locale.get_language()).and_then(|s| {
                        if s == "und" {
                            None
                        } else {
                            Some(s)
                        }
                    });
                    (
                        lang,
                        locale.get_script(),
                        locale.get_region(),
                        locale.get_variants(),
                        Some(locale.extensions.clone()),
                    )
                })
                .collect();
            b.iter(|| {
                for (language, script, region, variants, extensions) in &entries {
                    let _ = Locale::from_parts(
                        *language,
                        *script,
                        *region,
                        variants,
                        extensions.clone(),
                    );
                }
            })
        }),
        Fun::new("from_parts_unchecked", |b, locales: &Vec<Locale>| {
            let entries = locales
                .iter()
                .map(|locale| locale.clone().into_raw_parts())
                .collect::<Vec<_>>();
            b.iter(|| {
                for (language, script, region, variants, extensions) in &entries {
                    let _ = unsafe {
                        Locale::from_raw_parts_unchecked(
                            language.map(|l| TinyStr8::new_unchecked(l)),
                            script.map(|s| TinyStr4::new_unchecked(s)),
                            region.map(|r| TinyStr4::new_unchecked(r)),
                            variants.as_ref().map(|v| {
                                v.into_iter().map(|v| TinyStr8::new_unchecked(*v)).collect()
                            }),
                            extensions.parse().unwrap(),
                        )
                    };
                }
            })
        }),
    ];

    c.bench_functions("locale_construct", funcs, locales);
}

criterion_group!(benches, locale_construct_bench,);
criterion_main!(benches);
