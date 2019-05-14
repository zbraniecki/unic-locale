pub mod errors;

pub use self::errors::ParserError;
use crate::subtags;
use crate::LanguageIdentifier;

pub fn parse_language_identifier(t: &str) -> Result<LanguageIdentifier, ParserError> {
    let mut position = 0;

    let mut language = None;
    let mut script = None;
    let mut region = None;
    let mut variants = vec![];

    for subtag in t.split(|c| ['-', '_'].contains(&c)) {
        if position == 0 {
            // Language
            language = subtags::parse_language_subtag(subtag)?;
            position = 1;
            continue;
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

    Ok(LanguageIdentifier {
        language,
        script,
        region,
        variants,
    })
}
