pub mod errors;

pub use self::errors::ParserError;
use crate::subtags;
use crate::LanguageIdentifier;

static SEPARATORS: &[char] = &['-', '_'];

pub fn parse_language_identifier(
    t: &str,
    allow_extension: bool,
) -> Result<(LanguageIdentifier, Option<&str>), ParserError> {
    let mut position = 0;

    let mut language = None;
    let mut script = None;
    let mut region = None;
    let mut variants = vec![];

    let mut ptr = 0;
    let mut has_extension = false;

    for subtag in t.split(|c| SEPARATORS.contains(&c)) {
        let slen = subtag.len();

        ptr += slen + 1;

        if position == 0 {
            // Language
            language = subtags::parse_language_subtag(subtag)?;
            position = 1;
            continue;
        }

        if allow_extension && slen == 1 {
            has_extension = true;
            break;
        }

        if position == 1 {
            position = 2;
            // Script
            if let Ok(s) = subtags::parse_script_subtag(subtag) {
                script = Some(s);
                continue;
            }
        }

        if position == 2 {
            position = 3;
            // Region
            if let Ok(s) = subtags::parse_region_subtag(subtag) {
                region = Some(s);
                continue;
            }
        }

        if position == 3 {
            // Variants
            variants.push(subtags::parse_variant_subtag(subtag)?);
        }
    }

    variants.sort();
    variants.dedup();

    let exception = if has_extension {
        Some(&t[ptr - 2..])
    } else {
        None
    };

    Ok((
        LanguageIdentifier {
            language,
            script,
            region,
            variants: variants.into_boxed_slice(),
        },
        exception,
    ))
}
