pub mod errors;

pub use self::errors::ParserError;
use super::extensions;
use super::Locale;
use std::collections::HashMap;
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
            extensions: HashMap::new(),
        })
    }
}

fn parse_extension_subtags(
    t: &str,
) -> Result<extensions::ExtensionsMap, ParserError> {
    let mut result = HashMap::new();
    let mut current_type: Option<&mut HashMap<String, String>> = None;
    let mut current_key: Option<&str> = None;

    for subtag in t.split(|c: char| SEPARATORS.contains(&(c as u8))) {
        let slen = subtag.len();
        if slen == 1 {
            if let Some(current_key) = current_key.take() {
                if let Some(current_type) = current_type {
                    let key = extensions::convert_ext_key_to_key(current_key)?;
                    current_type.insert(key.to_string(), String::from("true"));
                } else {
                    return Err(ParserError::InvalidSubtag);
                }
            }
            let t = extensions::convert_str_to_ext_type(subtag)?;
            current_type = Some(result.entry(t).or_insert(HashMap::new()));
            continue;
        }

        if let Some(ref mut current_type) = current_type {
            if let Some(current_key) = current_key.take() {
                let key = extensions::convert_ext_key_to_key(current_key)?;
                current_type.insert(key.to_string(), subtag.to_string());
            } else {
                current_key = Some(subtag);
            }
        } else {
            return Err(ParserError::InvalidSubtag);
        }
    }
    if let Some(current_key) = current_key.take() {
        if let Some(current_type) = current_type {
            let key = extensions::convert_ext_key_to_key(current_key)?;
            current_type.insert(key.to_string(), String::from("true"));
        } else {
            return Err(ParserError::InvalidSubtag);
        }
    }
    Ok(result)
}
