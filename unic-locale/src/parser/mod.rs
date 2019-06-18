pub mod errors;

use std::convert::TryFrom;
pub use self::errors::ParserError;
use super::extensions::{ExtensionType, ExtensionsMap, UnicodeExtensionKey};
use super::Locale;
use unic_langid::parser::parse_language_identifier;

static SEPARATORS: &[u8] = &[b'-', b'_'];

fn extension_start(t: &str) -> Option<usize> {
    let mut ptr = 0;
    let bytes = t.as_bytes();
    let slen = bytes.len();

    while ptr < slen {
        let b = bytes[ptr];
        if SEPARATORS.contains(&b)
            && (slen > ptr + 1 && SEPARATORS.contains(&bytes[ptr + 2]))
            && bytes[ptr + 1].is_ascii_alphabetic()
        {
            return Some(ptr);
        }
        ptr += 1;
    }
    None
}

pub fn parse_locale(t: &str) -> Result<Locale, ParserError> {
    if let Some(pos) = extension_start(t) {
        let extensions = parse_extension_subtags(&t[pos + 1..].to_ascii_lowercase())?;
        Ok(Locale {
            langid: parse_language_identifier(&t[..pos])?,
            extensions,
        })
    } else {
        Ok(Locale {
            langid: parse_language_identifier(t)?,
            extensions: ExtensionsMap::default(),
        })
    }
}

pub fn parse_extension_subtags(t: &str) -> Result<ExtensionsMap, ParserError> {
    let mut result = ExtensionsMap::default();
    let mut current_type: Option<ExtensionType> = None;
    let mut current_key: Option<&str> = None;

    for subtag in t.split(|c: char| SEPARATORS.contains(&(c as u8))) {
        let slen = subtag.len();
        if slen == 1 {
            if let Some(current_key) = current_key.take() {
                if let Some(current_type) = current_type {
                    match current_type {
                        ExtensionType::Unicode => {
                            let key = UnicodeExtensionKey::try_from(current_key).map_err(|_| ParserError::InvalidExtension)?;
                            result.set_unicode_value(key, None).map_err(|_| ParserError::InvalidExtension)?
                        },
                        ExtensionType::Transform => {
                            result.set_transform_value(current_key, None).map_err(|_| ParserError::InvalidExtension)?
                        },
                        ExtensionType::Private => {
                            result.set_private_value(current_key, None).map_err(|_| ParserError::InvalidExtension)?
                        },
                    }
                } else {
                    return Err(ParserError::InvalidExtension);
                }
            }
            current_type = Some(ExtensionType::try_from(subtag).map_err(|_| ParserError::InvalidExtension)?);
            continue;
        }

        if let Some(current_type) = current_type {
            if let Some(current_key) = current_key.take() {
                match current_type {
                    ExtensionType::Unicode => {
                        let key = UnicodeExtensionKey::try_from(current_key).map_err(|_| ParserError::InvalidExtension)?;
                        result.set_unicode_value(key, Some(subtag)).map_err(|_| ParserError::InvalidExtension)?
                    },
                    ExtensionType::Transform => {
                        result.set_transform_value(current_key, Some(subtag)).map_err(|_| ParserError::InvalidExtension)?
                    },
                    ExtensionType::Private => {
                        result.set_private_value(current_key, Some(subtag)).map_err(|_| ParserError::InvalidExtension)?
                    },
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
                ExtensionType::Unicode => {
                    let key = UnicodeExtensionKey::try_from(current_key).map_err(|_| ParserError::InvalidExtension)?;
                    result.set_unicode_value(key, None).map_err(|_| ParserError::InvalidExtension)?
                },
                ExtensionType::Transform => {
                    result.set_transform_value(current_key, None).map_err(|_| ParserError::InvalidExtension)?
                },
                ExtensionType::Private => {
                    result.set_private_value(current_key, None).map_err(|_| ParserError::InvalidExtension)?
                },
            }
        } else {
            return Err(ParserError::InvalidSubtag);
        }
    }
    Ok(result)
}
