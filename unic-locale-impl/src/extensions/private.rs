use crate::errors::LocaleError;
use crate::parser::ParserError;

use tinystr::TinyStr8;

/// A list of [`Unicode Private Extensions`] as defined in [`Unicode Locale
/// Identifier`] specification.
///
/// Those extensions are intended for `pass-through` use.
///
/// # Examples
///
/// ```
/// use unic_locale_impl::Locale;
///
/// let mut loc: Locale = "en-US-x-foo-faa".parse()
///     .expect("Parsing failed.");
///
/// assert_eq!(loc.extensions.private.has_tag("faa"), Ok(true));
/// assert_eq!(loc.extensions.private.tags().next(), Some("faa")); // tags got sorted
/// loc.extensions.private.clear_tags();
/// assert_eq!(loc.to_string(), "en-US");
/// ```
///
/// [`Unicode Private Extensions`]: https://unicode.org/reports/tr35/#pu_extensions
/// [`Unicode Locale Identifier`]: https://unicode.org/reports/tr35/#Unicode_locale_identifier
#[derive(Clone, PartialEq, Eq, Debug, Default, Hash, PartialOrd, Ord)]
pub struct PrivateExtensionList(Vec<TinyStr8>);

fn parse_value(t: &[u8]) -> Result<TinyStr8, ParserError> {
    let s = TinyStr8::try_from_utf8(t).map_err(|_| ParserError::InvalidSubtag)?;
    if t.is_empty() || t.len() > 8 || !s.is_ascii_alphanumeric() {
        return Err(ParserError::InvalidSubtag);
    }

    Ok(s.to_ascii_lowercase())
}

impl PrivateExtensionList {
    /// Returns `true` if there are no tags in the PrivateExtensionList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "en-US-x-foo".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.extensions.private.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns `true` if tag is included in the `PrivateExtensionList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "en-US-x-foo".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.extensions.private.has_tag("foo")
    ///               .expect("Getting tag failed."),
    ///            true);
    /// ```
    pub fn has_tag<S: AsRef<[u8]>>(&self, tag: S) -> Result<bool, LocaleError> {
        Ok(self.0.contains(&parse_value(tag.as_ref())?))
    }

    /// Returns an iterator over all tags in the `PrivateExtensionList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "en-US-x-foo-bar".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.extensions.private.tags().collect::<Vec<_>>(),
    ///            &["bar", "foo"]);
    /// ```
    pub fn tags(&self) -> impl ExactSizeIterator<Item = &str> {
        self.0.iter().map(|s| s.as_ref())
    }

    /// Adds a tag to the `PrivateExtensionList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "en-US".parse()
    ///     .expect("Parsing failed.");
    ///
    /// loc.extensions.private.add_tag("foo")
    ///     .expect("Adding tag failed.");
    ///
    /// assert_eq!(loc.to_string(), "en-US-x-foo");
    /// ```
    pub fn add_tag<S: AsRef<[u8]>>(&mut self, tag: S) -> Result<(), LocaleError> {
        self.0.push(parse_value(tag.as_ref())?);
        self.0.sort_unstable();
        Ok(())
    }

    /// Removes a tag from the `PrivateExtensionList`.
    ///
    /// Returns `true` if tag was included in the `PrivateExtensionList` before
    /// removal.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "en-US-x-foo".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.extensions.private.remove_tag("foo")
    ///               .expect("Removing tag failed."),
    ///            true);
    ///
    /// assert_eq!(loc.to_string(), "en-US");
    /// ```
    pub fn remove_tag<S: AsRef<[u8]>>(&mut self, tag: S) -> Result<bool, LocaleError> {
        let value = parse_value(tag.as_ref())?;
        match self.0.binary_search(&value) {
            Ok(idx) => {
                self.0.remove(idx);
                Ok(true)
            }
            Err(_) => Ok(false),
        }
    }

    /// Clears all tags from the `PrivateExtensionList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "en-US-x-foo".parse()
    ///     .expect("Parsing failed.");
    ///
    /// loc.extensions.private.clear_tags();
    /// assert_eq!(loc.to_string(), "en-US");
    /// ```
    pub fn clear_tags(&mut self) {
        self.0.clear();
    }

    pub(crate) fn try_from_iter<'a>(
        iter: &mut impl Iterator<Item = &'a [u8]>,
    ) -> Result<Self, ParserError> {
        let mut pext = Self::default();

        for subtag in iter {
            pext.0.push(parse_value(subtag)?);
        }
        pext.0.sort_unstable();

        Ok(pext)
    }
}

impl std::fmt::Display for PrivateExtensionList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        f.write_str("-x")?;

        for subtag in &self.0 {
            write!(f, "-{}", subtag)?;
        }
        Ok(())
    }
}
