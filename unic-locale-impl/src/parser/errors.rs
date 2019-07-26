use std::error::Error;
use std::fmt::{self, Display};
use unic_langid_impl::parser::ParserError as LangIdParserError;

#[derive(Debug)]
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

impl Error for ParserError {
    fn description(&self) -> &str {
        match &self {
            ParserError::InvalidLanguage => "The given language subtag is invalid",
            ParserError::InvalidSubtag => "Invalid subtag",
            ParserError::InvalidExtension => "Invalid extension",
            ParserError::LangIdError(_) => "Language Identifier Parser Error",
        }
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}
