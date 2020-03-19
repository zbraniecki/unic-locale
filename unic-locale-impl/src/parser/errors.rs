use std::error::Error;
use std::fmt::{self, Display};
use unic_langid_impl::parser::ParserError as LangIdParserError;

#[derive(Debug, PartialEq)]
pub enum ParserError {
    InvalidLanguage,
    InvalidSubtag,
    InvalidExtension,
    LangIdError(LangIdParserError),
}

impl From<LangIdParserError> for ParserError {
    fn from(error: LangIdParserError) -> Self {
        ParserError::LangIdError(error)
    }
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match &self {
            ParserError::InvalidLanguage => "The given language subtag is invalid",
            ParserError::InvalidSubtag => "Invalid subtag",
            ParserError::InvalidExtension => "Invalid extension",
            ParserError::LangIdError(_) => "Language Identifier Parser Error",
        };
        f.write_str(value)
    }
}
