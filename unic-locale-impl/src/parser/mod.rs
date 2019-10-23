pub mod errors;

pub use self::errors::ParserError;
use super::extensions::ExtensionsMap;
use super::Locale;
use unic_langid_impl::LanguageIdentifier;

pub fn parse_locale<S: AsRef<[u8]>>(t: S) -> Result<Locale, ParserError> {
    let mut iter = t.as_ref().split(|c| *c == b'-' || *c == b'_').peekable();

    let langid = LanguageIdentifier::try_from_iter(&mut iter, true)
        .map_err(|_| ParserError::InvalidLanguage)?;

    let extensions = ExtensionsMap::try_from_iter(&mut iter)?;
    Ok(Locale { langid, extensions })
}
