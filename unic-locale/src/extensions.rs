use crate::parser::ParserError;

pub fn convert_ext_key_to_key(input: &str) -> Result<&str, ParserError> {
    if input == "hc" {
        return Ok("hour-cycle");
    }
    if input == "ca" {
        return Ok("calendar");
    }
    return Ok(input);
}

pub fn convert_key_to_ext_key(input: &str) -> Result<&str, ParserError> {
    if input == "hour-cycle" {
        return Ok("hc");
    }
    if input == "calendar" {
        return Ok("ca");
    }
    return Ok(input);
}

pub fn convert_ext_type_to_type(input: &str) -> Result<&str, ParserError> {
    if input == "u" {
        return Ok("unicode");
    } else if input == "x" {
        return Ok("private");
    }
    Err(ParserError::InvalidSubtag)
}

pub fn convert_type_to_ext_type(input: &str) -> Result<&str, ParserError> {
    if input == "unicode" {
        return Ok("u");
    } else if input == "private" {
        return Ok("x");
    }
    Err(ParserError::InvalidSubtag)
}
