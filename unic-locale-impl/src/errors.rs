use super::parser::ParserError;
use std::error::Error;
use std::fmt::{self, Display};
use unic_langid_impl::errors::LanguageIdentifierError;

#[derive(Debug)]
pub enum LocaleError {
    Unknown,
    ParserError(ParserError),
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

impl Error for LocaleError {
    fn description(&self) -> &str {
        match self {
            LocaleError::Unknown => "Unknown error",
            LocaleError::ParserError(p) => p.description(),
            LocaleError::LanguageIdentifierError(_) => "LangId Error",
        }
    }
}

impl Display for LocaleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}
