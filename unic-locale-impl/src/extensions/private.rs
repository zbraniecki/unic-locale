use crate::errors::LocaleError;
use crate::parser::ParserError;

use tinystr::TinyStr8;

#[derive(Clone, PartialEq, Eq, Debug, Default, Hash)]
pub struct PrivateExtensionList(Vec<TinyStr8>);

fn parse_value(t: &[u8]) -> Result<TinyStr8, ParserError> {
    let s = TinyStr8::from_bytes(t).map_err(|_| ParserError::InvalidSubtag)?;
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
    /// let mut lo: Locale = "en-US-x-foo".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(lo.extensions.private.is_empty(), false);
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
    /// let mut lo: Locale = "en-US-x-foo".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(lo.extensions.private.has_tag("foo")
    ///                .expect("Getting tag failed."),
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
    /// let mut lo: Locale = "en-US-x-foo-bar".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(lo.extensions.private.get_tags().collect::<Vec<_>>(),
    ///            &["bar", "foo"]);
    /// ```
    pub fn get_tags(&self) -> impl ExactSizeIterator<Item = &str> {
        self.0.iter().map(|s| s.as_ref())
    }

    /// Adds a tag to the `PrivateExtensionList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut lo: Locale = "en-US".parse()
    ///     .expect("Parsing failed.");
    ///
    /// lo.extensions.private.add_tag("foo")
    ///     .expect("Adding tag failed.");
    ///
    /// assert_eq!(lo.to_string(), "en-US-x-foo");
    /// ```
    pub fn add_tag<S: AsRef<[u8]>>(&mut self, tag: S) -> Result<(), LocaleError> {
        self.0.push(parse_value(tag.as_ref())?);
        self.0.sort();
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
    /// let mut lo: Locale = "en-US-x-foo".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(lo.extensions.private.remove_tag("foo")
    ///                .expect("Removing tag failed."),
    ///            true);
    ///
    /// assert_eq!(lo.to_string(), "en-US");
    /// ```
    pub fn remove_tag<S: AsRef<[u8]>>(&mut self, tag: S) -> Result<bool, LocaleError> {
        let value = parse_value(tag.as_ref())?;
        match self.0.binary_search(&value) {
            Ok(idx) => {
                self.0.remove(idx);
                Ok(true)
            },
            Err(_) => Ok(false)
        }
    }

    /// Clears all tags from the `PrivateExtensionList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut lo: Locale = "en-US-x-foo".parse()
    ///     .expect("Parsing failed.");
    ///
    /// lo.extensions.private.clear_tags();
    /// assert_eq!(lo.to_string(), "en-US");
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
        pext.0.sort();

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
