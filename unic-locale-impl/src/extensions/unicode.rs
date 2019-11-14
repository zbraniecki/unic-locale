use crate::errors::LocaleError;
use crate::parser::ParserError;

use std::collections::BTreeMap;
use std::iter::Peekable;

use tinystr::{TinyStr4, TinyStr8};

/// A list of [`Unicode BCP47 U Extensions`] as defined in [`Unicode Locale
/// Identifier`] specification.
///
/// Unicode extensions provide subtags that specify language and/or locale-based behavior
/// or refinements to language tags, according to work done by the Unicode Consortium.
/// (See [`RFC 6067`] for details).
///
/// # Examples
///
/// ```
/// use unic_locale_impl::Locale;
///
/// let mut loc: Locale = "de-u-hc-h12-ca-buddhist".parse()
///     .expect("Parsing failed.");
///
/// assert_eq!(loc.extensions.unicode.get_keyword("ca")
///               .expect("Getting keyword failed.")
///               .collect::<Vec<_>>(),
///            &["buddhist"]);
/// ```
/// [`Unicode BCP47 U Extensions`]: https://unicode.org/reports/tr35/#u_Extension
/// [`RFC 6067`]: https://www.ietf.org/rfc/rfc6067.txt
/// [`Unicode Locale Identifier`]: https://unicode.org/reports/tr35/#Unicode_locale_identifier
#[derive(Clone, PartialEq, Eq, Debug, Default, Hash, PartialOrd, Ord)]
pub struct UnicodeExtensionList {
    // Canonical: sort by key (BTreeMap is already sorted) / remove value 'true'
    keywords: BTreeMap<TinyStr4, Vec<TinyStr8>>,

    // Canonical: sort / de-dup
    attributes: Vec<TinyStr8>,
}

fn parse_key(key: &[u8]) -> Result<TinyStr4, ParserError> {
    if key.len() != 2 || !key[0].is_ascii_alphanumeric() || !key[1].is_ascii_alphabetic() {
        return Err(ParserError::InvalidSubtag);
    }
    let key = TinyStr4::from_bytes(key).map_err(|_| ParserError::InvalidSubtag)?;
    Ok(key.to_ascii_lowercase())
}

const TRUE_TYPE: TinyStr8 = unsafe { TinyStr8::new_unchecked(1_702_195_828u64) }; // "true"

fn parse_type(t: &[u8]) -> Result<Option<TinyStr8>, ParserError> {
    let s = TinyStr8::from_bytes(t).map_err(|_| ParserError::InvalidSubtag)?;
    if t.len() < 3 || t.len() > 8 || !s.is_ascii_alphanumeric() {
        return Err(ParserError::InvalidSubtag);
    }

    let s = s.to_ascii_lowercase();

    if s == TRUE_TYPE {
        Ok(None)
    } else {
        Ok(Some(s))
    }
}

fn parse_attribute(t: &[u8]) -> Result<TinyStr8, ParserError> {
    let s = TinyStr8::from_bytes(t).map_err(|_| ParserError::InvalidSubtag)?;
    if t.len() < 3 || t.len() > 8 || !s.is_ascii_alphanumeric() {
        return Err(ParserError::InvalidSubtag);
    }

    Ok(s.to_ascii_lowercase())
}

fn is_attribute(t: &[u8]) -> bool {
    let slen = t.len();
    (slen >= 3 && slen <= 8) && !t.iter().any(|c: &u8| !c.is_ascii_alphanumeric())
}

fn is_type(t: &[u8]) -> bool {
    let slen = t.len();
    (slen >= 3 && slen <= 8) && !t.iter().any(|c: &u8| !c.is_ascii_alphanumeric())
}

impl UnicodeExtensionList {
    /// Returns `true` if there are no keywords and no attributes in
    /// the `UnicodeExtensionList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "en-US-u-foo".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.extensions.unicode.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.keywords.is_empty() && self.attributes.is_empty()
    }

    /// Returns the value of keyword in the `UnicodeExtensionList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "en-US-u-ca-buddhist".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.extensions.unicode.get_keyword("ca")
    ///                .expect("Getting keyword failed.")
    ///                .collect::<Vec<_>>(),
    ///            &["buddhist"]);
    ///
    /// // Here keyword with key "aa" is not available
    /// assert_eq!(loc.extensions.unicode.get_keyword("aa")
    ///               .expect("Getting keyword failed.")
    ///               .len(),
    ///            0);
    /// ```
    pub fn get_keyword<S: AsRef<[u8]>>(
        &self,
        key: S,
    ) -> Result<impl ExactSizeIterator<Item = &str>, LocaleError> {
        let keywords: &[_] = match self.keywords.get(&parse_key(key.as_ref())?) {
            Some(ref v) => &**v,
            None => &[],
        };

        Ok(keywords.iter().map(|s| s.as_ref()))
    }

    /// Returns an iterator over all keys in the `UnicodeExtensionList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "en-US-u-ca-buddhist-nu-thai".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.extensions.unicode.get_keyword_keys().collect::<Vec<_>>(),
    ///            &["ca", "nu"]);
    /// ```
    pub fn get_keyword_keys(&self) -> impl ExactSizeIterator<Item = &str> {
        self.keywords.keys().map(|s| s.as_ref())
    }

    /// Adds a keyword to the `UnicodeExtensionList` or sets value for key if
    /// keyword is already included in the `UnicodeExtensionList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "en-US".parse()
    ///     .expect("Parsing failed.");
    ///
    /// loc.extensions.unicode.set_keyword("ca", &["buddhist"])
    ///     .expect("Setting keyword failed.");
    ///
    /// assert_eq!(loc.to_string(), "en-US-u-ca-buddhist");
    ///
    /// loc.extensions.unicode.set_keyword("ca", &["chinese"])
    ///     .expect("Setting keyword failed.");
    ///
    /// assert_eq!(loc.to_string(), "en-US-u-ca-chinese");
    /// ```
    pub fn set_keyword<S: AsRef<[u8]>>(&mut self, key: S, value: &[S]) -> Result<(), LocaleError> {
        let key = parse_key(key.as_ref())?;

        let t = value
            .iter()
            .filter_map(|t| parse_type(t.as_ref()).transpose())
            .collect::<Result<Vec<_>, _>>()?;

        self.keywords.insert(key, t);
        Ok(())
    }

    /// Removes a keyword from the `UnicodeExtensionList`.
    ///
    /// Returns `true` if keyword was included in the `UnicodeExtensionList`
    /// before removal.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "en-US-u-ca-buddhist".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.extensions.unicode.remove_keyword("ca")
    ///               .expect("Removing tag failed."),
    ///            true);
    ///
    /// assert_eq!(loc.to_string(), "en-US");
    /// ```
    pub fn remove_keyword<S: AsRef<[u8]>>(&mut self, key: S) -> Result<bool, LocaleError> {
        Ok(self.keywords.remove(&parse_key(key.as_ref())?).is_some())
    }

    /// Clears all keywords from the `UnicodeExtensionList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "en-US-u-ca-buddhist".parse()
    ///     .expect("Parsing failed.");
    ///
    /// loc.extensions.unicode.clear_keywords();
    /// assert_eq!(loc.to_string(), "en-US");
    /// ```
    pub fn clear_keywords(&mut self) {
        self.keywords.clear();
    }

    /// Returns `true` if attribute is included in the `UnicodeExtensionList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "en-US-u-foo".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.extensions.unicode.has_attribute("foo")
    ///               .expect("Getting attribute failed."),
    ///            true);
    /// ```
    pub fn has_attribute<S: AsRef<[u8]>>(&self, attribute: S) -> Result<bool, LocaleError> {
        Ok(self
            .attributes
            .contains(&parse_attribute(attribute.as_ref())?))
    }

    /// Returns an iterator over all attributes in the `UnicodeExtensionList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "en-US-u-foo-bar".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.extensions.unicode.get_attributes().collect::<Vec<_>>(),
    ///            &["bar", "foo"]);
    /// ```
    pub fn get_attributes(&self) -> impl ExactSizeIterator<Item = &str> {
        self.attributes.iter().map(|s| s.as_ref())
    }

    /// Sets an attribute on the `UnicodeExtensionList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "en-US".parse()
    ///     .expect("Parsing failed.");
    ///
    /// loc.extensions.unicode.set_attribute("foo")
    ///     .expect("Setting attribute failed.");
    ///
    /// assert_eq!(loc.to_string(), "en-US-u-foo");
    /// ```
    pub fn set_attribute<S: AsRef<[u8]>>(&mut self, attribute: S) -> Result<(), LocaleError> {
        let attribute = parse_attribute(attribute.as_ref())?;
        if let Err(idx) = self.attributes.binary_search(&attribute) {
            self.attributes.insert(idx, attribute);
        }
        Ok(())
    }

    /// Removes an attribute from the `UnicodeExtensionList`.
    ///
    /// Returns `true` if attribute was included in the `UnicodeExtensionList`
    /// before removal.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "en-US-u-foo".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.extensions.unicode.remove_attribute("foo")
    ///               .expect("Removing attribute failed."),
    ///            true);
    ///
    /// assert_eq!(loc.to_string(), "en-US");
    /// ```
    pub fn remove_attribute<S: AsRef<[u8]>>(&mut self, attribute: S) -> Result<bool, LocaleError> {
        let attribute = parse_attribute(attribute.as_ref())?;
        match self.attributes.binary_search(&attribute) {
            Ok(idx) => {
                self.attributes.remove(idx);
                Ok(true)
            }
            Err(_) => Ok(false),
        }
    }

    /// Clears all attributes from the `UnicodeExtensionList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "en-US-u-foo".parse()
    ///     .expect("Parsing failed.");
    ///
    /// loc.extensions.unicode.clear_attributes();
    /// assert_eq!(loc.to_string(), "en-US");
    /// ```
    pub fn clear_attributes(&mut self) {
        self.attributes.clear();
    }

    pub(crate) fn try_from_iter<'a>(
        iter: &mut Peekable<impl Iterator<Item = &'a [u8]>>,
    ) -> Result<Self, ParserError> {
        let mut uext = Self::default();

        let mut st_peek = iter.peek();

        let mut current_keyword = None;
        let mut current_types = vec![];

        while let Some(subtag) = st_peek {
            let slen = subtag.len();
            if slen == 2 {
                if let Some(current_keyword) = current_keyword {
                    uext.keywords.insert(current_keyword, current_types);
                    current_types = vec![];
                }
                current_keyword = Some(parse_key(subtag)?);
                iter.next();
            } else if current_keyword.is_some() && is_type(subtag) {
                if let Some(ty) = parse_type(subtag)? {
                    current_types.push(ty);
                }
                iter.next();
            } else if is_attribute(subtag) {
                uext.attributes.push(parse_attribute(subtag)?);
                iter.next();
            } else {
                break;
            }
            st_peek = iter.peek();
        }

        if let Some(current_keyword) = current_keyword {
            uext.keywords.insert(current_keyword, current_types);
        }

        uext.attributes.sort();
        uext.attributes.dedup();

        Ok(uext)
    }
}

impl std::fmt::Display for UnicodeExtensionList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        f.write_str("-u")?;

        for attr in &self.attributes {
            write!(f, "-{}", attr)?;
        }

        for (k, t) in &self.keywords {
            write!(f, "-{}", k)?;
            for v in t {
                write!(f, "-{}", v)?;
            }
        }
        Ok(())
    }
}
