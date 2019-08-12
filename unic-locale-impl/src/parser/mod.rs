pub mod errors;

pub use self::errors::ParserError;
use super::extensions::{ExtensionType, ExtensionsMap};
use super::Locale;
use unic_langid_impl::parser::parse_language_identifier;

static SEPARATORS: &[char] = &['-', '_'];

pub fn parse_locale(t: &str) -> Result<Locale, ParserError> {
    let (langid, ext_str) = parse_language_identifier(t, true)?;

    let extensions = if let Some(ext_str) = ext_str {
        parse_extension_subtags(&ext_str.to_ascii_lowercase())?
    } else {
        ExtensionsMap::default()
    };
    Ok(Locale { langid, extensions })
}

pub fn parse_extension_subtags(t: &str) -> Result<ExtensionsMap, ParserError> {
    let mut result = ExtensionsMap::default();
    if t.is_empty() {
        return Ok(result);
    }

    let mut current_type: Option<ExtensionType> = None;
    let mut current_key: Option<&str> = None;

    for subtag in t.split(|c| SEPARATORS.contains(&c)) {
        let slen = subtag.len();
        if slen == 1 {
            if let Some(current_key) = current_key.take() {
                if let Some(current_type) = current_type {
                    match current_type {
                        ExtensionType::Unicode => result
                            .set_unicode_value(current_key, None)
                            .map_err(|_| ParserError::InvalidExtension)?,
                        ExtensionType::Transform => result
                            .set_transform_value(current_key, None)
                            .map_err(|_| ParserError::InvalidExtension)?,
                        ExtensionType::Other(_) => unimplemented!(),
                        ExtensionType::Private => result
                            .set_private_value(current_key, None)
                            .map_err(|_| ParserError::InvalidExtension)?,
                    }
                } else {
                    return Err(ParserError::InvalidExtension);
                }
            }
            current_type = Some(ExtensionType::from_char(subtag.chars().nth(0).unwrap())?);
            continue;
        }

        if let Some(current_type) = current_type {
            if let Some(current_key) = current_key.take() {
                match current_type {
                    ExtensionType::Unicode => result
                        .set_unicode_value(current_key, Some(subtag))
                        .map_err(|_| ParserError::InvalidExtension)?,
                    ExtensionType::Transform => result
                        .set_transform_value(current_key, Some(subtag))
                        .map_err(|_| ParserError::InvalidExtension)?,
                    ExtensionType::Other(_) => unimplemented!(),
                    ExtensionType::Private => result
                        .set_private_value(current_key, Some(subtag))
                        .map_err(|_| ParserError::InvalidExtension)?,
                }
            } else {
                current_key = Some(subtag);
            }
        } else {
            return Err(ParserError::InvalidSubtag);
        }
    }
    if let Some(current_key) = current_key.take() {
        if let Some(current_type) = current_type {
            match current_type {
                ExtensionType::Unicode => result
                    .set_unicode_value(current_key, None)
                    .map_err(|_| ParserError::InvalidExtension)?,
                ExtensionType::Transform => result
                    .set_transform_value(current_key, None)
                    .map_err(|_| ParserError::InvalidExtension)?,
                ExtensionType::Other(_) => unimplemented!(),
                ExtensionType::Private => result
                    .set_private_value(current_key, None)
                    .map_err(|_| ParserError::InvalidExtension)?,
            }
        } else {
            return Err(ParserError::InvalidSubtag);
        }
    }
    Ok(result)
}
