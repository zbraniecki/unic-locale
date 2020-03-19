use crate::parser::ParserError;
use std::error::Error;
use std::fmt::{self, Display};
use unic_langid_impl::LanguageIdentifierError;

/// Enum with errors that can be returned by Locale.
#[derive(Debug, PartialEq)]
pub enum LocaleError {
    /// An unknown error - currently covers all-but parser errors.
    Unknown,
    /// A parser error.
    ParserError(ParserError),
    /// An error from parsing LanguageIdentifier portion.
    LanguageIdentifierError(LanguageIdentifierError),
}

impl From<ParserError> for LocaleError {
    fn from(error: ParserError) -> LocaleError {
        LocaleError::ParserError(error)
    }
}

impl From<LanguageIdentifierError> for LocaleError {
    fn from(error: LanguageIdentifierError) -> LocaleError {
        LocaleError::LanguageIdentifierError(error)
    }
}

impl Error for LocaleError {}

impl Display for LocaleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown error"),
            Self::ParserError(p) => write!(f, "Parser error: {}", p),
            Self::LanguageIdentifierError(l) => write!(f, "Language Identifier Error: {}", l),
        }
    }
}
