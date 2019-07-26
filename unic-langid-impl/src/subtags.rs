use crate::parser::errors::ParserError;
use std::borrow::Cow;

pub fn parse_language_subtag(subtag: &str) -> Result<Option<Cow<'static, str>>, ParserError> {
    let slen = subtag.len();

    if slen < 2 || slen > 8 || slen == 4 || subtag.contains(|c: char| !c.is_ascii_alphabetic()) {
        return Err(ParserError::InvalidLanguage);
    }

    let value = subtag.to_ascii_lowercase();

    if value == "und" {
        Ok(None)
    } else {
        Ok(Some(Cow::from(value)))
    }
}

pub fn parse_script_subtag(subtag: &str) -> Result<Cow<'static, str>, ParserError> {
    let slen = subtag.len();

    if slen != 4 || subtag.contains(|c: char| !c.is_ascii_alphabetic()) {
        return Err(ParserError::InvalidSubtag);
    }
    let mut result = subtag.to_ascii_lowercase();
    result[0..1].make_ascii_uppercase();
    Ok(result.into())
}

pub fn parse_region_subtag(subtag: &str) -> Result<Cow<'static, str>, ParserError> {
    let slen = subtag.len();

    if slen == 2 && !subtag.contains(|c: char| !c.is_ascii_alphabetic())
        || slen == 3 && !subtag.contains(|c: char| !c.is_ascii_digit())
    {
        Ok(subtag.to_ascii_uppercase().into())
    } else {
        Err(ParserError::InvalidSubtag)
    }
}

pub fn parse_variant_subtag(subtag: &str) -> Result<Cow<'static, str>, ParserError> {
    let slen = subtag.len();

    if slen < 4 || slen > 8 {
        return Err(ParserError::InvalidSubtag);
    }

    if slen >= 5 && subtag.contains(|c: char| !c.is_ascii_alphanumeric()) {
        return Err(ParserError::InvalidSubtag);
    }

    if slen == 4
        && !subtag.as_bytes()[0].is_ascii_digit()
        && subtag[1..].contains(|c: char| !c.is_ascii_alphanumeric())
    {
        return Err(ParserError::InvalidSubtag);
    }

    Ok(subtag.to_ascii_lowercase().into())
}
