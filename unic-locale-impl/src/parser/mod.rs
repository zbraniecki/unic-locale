pub mod errors;

pub use self::errors::ParserError;
use super::extensions::ExtensionsMap;
use super::Locale;
use unic_langid_impl::LanguageIdentifier;

static SEPARATORS: &[char] = &['-', '_'];

pub fn parse_locale(t: &str) -> Result<Locale, ParserError> {
    let mut iter = t.split(|c| SEPARATORS.contains(&c)).peekable();

    let langid = LanguageIdentifier::try_from_iter(&mut iter, true)
        .map_err(|_| ParserError::InvalidLanguage)?;

    let extensions = ExtensionsMap::try_from_iter(&mut iter)?;
    Ok(Locale { langid, extensions })
}
