use crate::parser::ParserError;
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum ExtensionType {
    Unicode,
    Transform,
    Private,
}

pub type ExtensionsMap = HashMap<ExtensionType, HashMap<String, String>>;

pub fn convert_str_to_ext_type(input: &str) -> Result<ExtensionType, ParserError> {
    match input {
        "u" => Ok(ExtensionType::Unicode),
        "t" => Ok(ExtensionType::Transform),
        "x" => Ok(ExtensionType::Private),
        _ => Err(ParserError::InvalidSubtag),
    }
}

pub fn convert_ext_type_to_string(input: &ExtensionType) -> &'static str {
    match input {
        ExtensionType::Unicode => "u",
        ExtensionType::Transform => "t",
        ExtensionType::Private => "x",
    }
}

pub fn convert_ext_key_to_key(input: &str) -> Result<&str, ParserError> {
    if input == "hc" {
        return Ok("hour-cycle");
    }
    if input == "ca" {
        return Ok("calendar");
    }
    Ok(input)
}

pub fn convert_key_to_ext_key(input: &str) -> Result<&str, ParserError> {
    if input == "hour-cycle" {
        return Ok("hc");
    }
    if input == "calendar" {
        return Ok("ca");
    }
    Ok(input)
}
