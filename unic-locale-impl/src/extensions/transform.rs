use crate::errors::LocaleError;
use crate::parser::ParserError;

use unic_langid_impl::LanguageIdentifier;

use std::collections::BTreeMap;
use std::iter::Peekable;

use tinystr::{TinyStr4, TinyStr8};

/// A list of [`Unicode BCP47 T Extensions`] as defined in [`Unicode Locale
/// Identifier`] specification.
///
/// Transform extension carries information about source language or script of
/// transformed content, including content that has been transliterated, transcribed,
/// or translated, or in some other way influenced by the source (See [`RFC 6497`] for details).
///
/// # Examples
///
/// ```
/// use unic_locale_impl::{Locale, LanguageIdentifier};
///
/// let mut loc: Locale = "de-t-en-US-h0-hybrid".parse()
///     .expect("Parsing failed.");
///
/// let en_us: LanguageIdentifier = "en-US".parse()
///     .expect("Parsing failed.");
///
/// assert_eq!(loc.extensions.transform.tlang(), Some(&en_us));
/// assert_eq!(
///     loc.extensions.transform.tfield("h0")
///                             .expect("Getting tfield failed.")
///                             .collect::<Vec<_>>(),
///     &["hybrid"]
/// );
/// ```
/// [`Unicode BCP47 T Extensions`]: https://unicode.org/reports/tr35/#t_Extension
/// [`RFC 6497`]: https://www.ietf.org/rfc/rfc6497.txt
/// [`Unicode Locale Identifier`]: https://unicode.org/reports/tr35/#Unicode_locale_identifier
#[derive(Clone, PartialEq, Eq, Debug, Default, Hash, PartialOrd, Ord)]
pub struct TransformExtensionList {
    tlang: Option<LanguageIdentifier>,

    // Canonical: sort by key (BTreeMap is already) / remove value 'true'
    tfields: BTreeMap<TinyStr4, Vec<TinyStr8>>,
}

fn parse_tkey(key: &[u8]) -> Result<TinyStr4, ParserError> {
    if key.len() != 2 || !key[0].is_ascii_alphabetic() || !key[1].is_ascii_digit() {
        return Err(ParserError::InvalidSubtag);
    }
    let tkey = TinyStr4::from_bytes(key).map_err(|_| ParserError::InvalidSubtag)?;
    Ok(tkey.to_ascii_lowercase())
}

const TRUE_TVALUE: TinyStr8 = unsafe { TinyStr8::new_unchecked(1_702_195_828u64) }; // "true"

fn parse_tvalue(t: &[u8]) -> Result<Option<TinyStr8>, ParserError> {
    let s = TinyStr8::from_bytes(t).map_err(|_| ParserError::InvalidSubtag)?;
    if t.len() < 3 || t.len() > 8 || !s.is_ascii_alphanumeric() {
        return Err(ParserError::InvalidSubtag);
    }

    let s = s.to_ascii_lowercase();

    if s == TRUE_TVALUE {
        Ok(None)
    } else {
        Ok(Some(s))
    }
}

fn is_language_subtag(t: &[u8]) -> bool {
    let slen = t.len();
    (slen >= 2 && slen <= 8 || slen == 4) && !t.iter().any(|c: &u8| !c.is_ascii_alphabetic())
}

impl TransformExtensionList {
    /// Returns `true` if there are no tfields and no tlang in
    /// the `TransformExtensionList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "en-US-t-es-AR".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.extensions.transform.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.tlang.is_none() && self.tfields.is_empty()
    }

    /// Gets tlang from the `TransformExtensionList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    /// use unic_langid_impl::LanguageIdentifier;
    ///
    /// let mut loc: Locale = "en-US-t-es-AR".parse()
    ///     .expect("Parsing failed.");
    ///
    /// let tlang: LanguageIdentifier = "es-AR".parse()
    ///     .expect("Parsing failed on tlang.");
    ///
    /// assert_eq!(loc.extensions.transform.tlang(), Some(&tlang));
    /// ```
    pub fn tlang(&self) -> Option<&LanguageIdentifier> {
        self.tlang.as_ref()
    }

    /// Sets tlang on the `TransformExtensionList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    /// use unic_langid_impl::LanguageIdentifier;
    ///
    /// let mut loc: Locale = "en-US".parse()
    ///     .expect("Parsing failed.");
    ///
    /// let tlang: LanguageIdentifier = "es-AR".parse()
    ///     .expect("Parsing failed on tlang.");
    ///
    /// loc.extensions.transform.set_tlang(tlang)
    ///     .expect("Setting tlang failed.");
    ///
    /// assert_eq!(loc.to_string(), "en-US-t-es-AR");
    /// ```
    pub fn set_tlang(&mut self, tlang: LanguageIdentifier) -> Result<(), LocaleError> {
        self.tlang = Some(tlang);
        Ok(())
    }

    /// Clears tlang on the `TransformExtensionList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    /// use unic_langid_impl::LanguageIdentifier;
    ///
    /// let mut loc: Locale = "en-US-t-es-AR".parse()
    ///     .expect("Parsing failed.");
    ///
    /// loc.extensions.transform.clear_tlang();
    ///
    /// assert_eq!(loc.to_string(), "en-US");
    /// ```
    pub fn clear_tlang(&mut self) {
        self.tlang = None;
    }

    /// Returns the tvalue of tfield in the `TransformExtensionList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "en-US-t-k0-dvorak".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.extensions.transform.tfield("k0")
    ///                .expect("Getting tfield failed.")
    ///                .collect::<Vec<_>>(),
    ///            &["dvorak"]);
    ///
    /// // Here tfield with tkey "m0" is not available
    /// assert_eq!(loc.extensions.transform.tfield("m0")
    ///                .expect("Getting tfield failed.")
    ///                .collect::<Vec<_>>()
    ///                .is_empty(),
    ///            true);
    /// ```
    pub fn tfield<S: AsRef<[u8]>>(
        &self,
        tkey: S,
    ) -> Result<impl ExactSizeIterator<Item = &str>, LocaleError> {
        let tfields: &[_] = match self.tfields.get(&parse_tkey(tkey.as_ref())?) {
            Some(ref v) => &**v,
            None => &[],
        };

        Ok(tfields.iter().map(|s| s.as_ref()))
    }

    /// Returns an iterator over all tkeys in the `TransformExtensionList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "en-US-t-k0-dvorak-h0-hybrid".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.extensions.transform.tfield_keys().collect::<Vec<_>>(),
    ///            &["h0", "k0"]);
    /// ```
    pub fn tfield_keys(&self) -> impl ExactSizeIterator<Item = &str> {
        self.tfields.keys().map(|s| s.as_ref())
    }

    /// Adds a tfield to the `TransformExtensionList` or sets tvalue for tkey if
    /// tfield is already included in the `TransformExtensionList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "en-US".parse()
    ///     .expect("Parsing failed.");
    ///
    /// loc.extensions.transform.set_tfield("k0", &["dvorak"])
    ///     .expect("Setting tfield failed.");
    ///
    /// assert_eq!(loc.to_string(), "en-US-t-k0-dvorak");
    ///
    /// loc.extensions.transform.set_tfield("k0", &["colemak"])
    ///     .expect("Setting tfield failed.");
    ///
    /// assert_eq!(loc.to_string(), "en-US-t-k0-colemak");
    /// ```
    pub fn set_tfield<S: AsRef<[u8]>>(&mut self, tkey: S, tvalue: &[S]) -> Result<(), LocaleError> {
        let tkey = parse_tkey(tkey.as_ref())?;

        let t = tvalue
            .iter()
            .filter_map(|val| parse_tvalue(val.as_ref()).transpose())
            .collect::<Result<Vec<_>, _>>()?;

        self.tfields.insert(tkey, t);
        Ok(())
    }

    /// Removes a tfield from the `TransformExtensionList`.
    ///
    /// Returns `true` if tfield was included in the `TransformExtensionList`
    /// before removal.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "en-US-t-k0-dvorak".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.extensions.transform.remove_tfield("k0")
    ///                .expect("Removing tfield failed."),
    ///            true);
    ///
    /// assert_eq!(loc.to_string(), "en-US");
    /// ```
    pub fn remove_tfield<S: AsRef<[u8]>>(&mut self, tkey: S) -> Result<bool, LocaleError> {
        Ok(self.tfields.remove(&parse_tkey(tkey.as_ref())?).is_some())
    }

    /// Clears all tfields from the `TransformExtensionList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "en-US-t-k0-dvorak".parse()
    ///     .expect("Parsing failed.");
    ///
    /// loc.extensions.transform.clear_tfields();
    /// assert_eq!(loc.to_string(), "en-US");
    /// ```
    pub fn clear_tfields(&mut self) {
        self.tfields.clear();
    }

    pub(crate) fn try_from_iter<'a>(
        mut iter: &mut Peekable<impl Iterator<Item = &'a [u8]>>,
    ) -> Result<Self, ParserError> {
        let mut text = Self::default();

        let mut st_peek = iter.peek();

        let mut current_tkey = None;
        let mut current_tvalue = vec![];

        while let Some(subtag) = st_peek {
            let slen = subtag.len();
            if slen == 2 && subtag[0].is_ascii_alphabetic() && subtag[1].is_ascii_digit() {
                if let Some(current_tkey) = current_tkey {
                    text.tfields.insert(current_tkey, current_tvalue);
                    current_tvalue = vec![];
                }
                current_tkey = Some(parse_tkey(subtag)?);
                iter.next();
            } else if current_tkey.is_some() {
                if let Some(tval) = parse_tvalue(subtag)? {
                    current_tvalue.push(tval);
                }
                iter.next();
            } else if is_language_subtag(subtag) {
                text.tlang = Some(
                    LanguageIdentifier::try_from_iter(&mut iter, true)
                        .map_err(|_| ParserError::InvalidLanguage)?,
                );
            } else {
                break;
            }
            st_peek = iter.peek();
        }

        if let Some(current_keyword) = current_tkey {
            text.tfields.insert(current_keyword, current_tvalue);
        }

        Ok(text)
    }
}

impl std::fmt::Display for TransformExtensionList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        f.write_str("-t")?;

        if let Some(tlang) = &self.tlang {
            write!(f, "-{}", tlang)?;
        }

        for (k, t) in &self.tfields {
            write!(f, "-{}", k)?;
            for v in t {
                write!(f, "-{}", v)?;
            }
        }
        Ok(())
    }
}
