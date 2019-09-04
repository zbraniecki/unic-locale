pub mod errors;

use std::iter::Peekable;

pub use self::errors::ParserError;
use crate::subtags;
use crate::LanguageIdentifier;

static SEPARATORS: &[char] = &['-', '_'];

pub fn parse_language_identifier_from_iter<'a>(
    iter: &mut Peekable<impl Iterator<Item = &'a str>>,
    allow_extension: bool,
) -> Result<LanguageIdentifier, ParserError> {
    let mut position = 0;

    let mut language = None;
    let mut script = None;
    let mut region = None;
    let mut variants = vec![];

    while let Some(subtag) = iter.next() {
        if position == 0 {
            // Language
            language = subtags::parse_language_subtag(subtag)?;
            position = 1;
        } else if position == 1 {
            if let Ok(s) = subtags::parse_script_subtag(subtag) {
                script = Some(s);
                position = 2;
            } else if let Ok(s) = subtags::parse_region_subtag(subtag) {
                region = Some(s);
                position = 3;
            } else {
                variants.push(subtags::parse_variant_subtag(subtag)?);
                position = 3;
            }
        } else if position == 2 {
            if let Ok(s) = subtags::parse_region_subtag(subtag) {
                region = Some(s);
                position = 3;
            } else {
                variants.push(subtags::parse_variant_subtag(subtag)?);
                position = 3;
            }
        } else {
            // Variants
            variants.push(subtags::parse_variant_subtag(subtag)?);
        }

        if allow_extension {
            if let Some(st_peek) = iter.peek() {
                if st_peek.len() == 1 {
                    break;
                }
            }
        }
    }

    let variants = if variants.is_empty() {
        None
    } else {
        variants.sort();
        variants.dedup();
        Some(variants.into_boxed_slice())
    };

    Ok(LanguageIdentifier {
        language,
        script,
        region,
        variants,
    })
}

pub fn parse_language_identifier(t: &str) -> Result<LanguageIdentifier, ParserError> {
    let mut iter = t.split(|c| SEPARATORS.contains(&c)).peekable();
    parse_language_identifier_from_iter(&mut iter, false)
}
