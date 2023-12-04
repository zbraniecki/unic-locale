pub(crate) mod errors;
pub mod extensions;
pub mod parser;

use errors::LocaleError;
pub use extensions::{ExtensionType, ExtensionsMap};
use std::str::FromStr;
pub use unic_langid_impl::CharacterDirection;
pub use unic_langid_impl::{subtags, LanguageIdentifier};

/// `Locale` is a core struct representing a Unicode Locale Identifier.
///
/// A locale is made of two parts:
///  * `id` - Unicode Language Identifier
///  * `extensions` - A set of Unicode Extensions
///
/// `Locale` exposes all of the same methods as `LanguageIdentifier`, and
/// on top of that is able to parse, manipulate and serialize unicode extension
/// fields.
///
/// # Examples
///
/// ```
/// use unic_locale_impl::Locale;
///
/// let loc: Locale = "en-US-u-ca-buddhist".parse()
///     .expect("Failed to parse.");
///
/// assert_eq!(loc.id.language, "en");
/// assert_eq!(loc.id.script, None);
/// assert_eq!(loc.id.region, Some("US".parse().unwrap()));
/// assert_eq!(loc.id.variants().len(), 0);
/// assert_eq!(loc.extensions.unicode.keyword("ca")
///     .expect("Getting keyword failed.")
///     .collect::<Vec<_>>(),
///     &["buddhist"]);
/// ```
///
/// # Parsing
///
/// Unicode recognizes three levels of standard conformance for a locale:
///
///  * *well-formed* - syntactically correct
///  * *valid* - well-formed and only uses registered language subtags, extensions, keywords, types...
///  * *canonical* - valid and no deprecated codes or structure.
///
/// At the moment parsing normalizes a well-formed language identifier converting
/// `_` separators to `-` and adjusting casing to conform to the Unicode standard.
///
/// Any bogus subtags will cause the parsing to fail with an error.
/// No subtag validation is performed.
///
/// # Examples:
///
/// ```
/// use unic_locale_impl::Locale;
///
/// let loc: Locale = "eN_latn_Us-Valencia_u-hC-H12".parse()
///     .expect("Failed to parse.");
///
/// assert_eq!(loc.id.language, "en");
/// assert_eq!(loc.id.script, Some("Latn".parse().unwrap()));
/// assert_eq!(loc.id.region, Some("US".parse().unwrap()));
/// assert_eq!(loc.id.variants().collect::<Vec<_>>(), &["valencia"]);
/// ```
#[derive(Debug, Default, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Locale {
    pub id: LanguageIdentifier,
    pub extensions: extensions::ExtensionsMap,
}

type PartsTuple = (
    subtags::Language,
    Option<subtags::Script>,
    Option<subtags::Region>,
    Vec<subtags::Variant>,
    String,
);

impl Locale {
    /// A constructor which takes a utf8 slice, parses it and
    /// produces a well-formed `Locale`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let loc = Locale::from_bytes("en-US-u-hc-h12".as_bytes())
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.to_string(), "en-US-u-hc-h12");
    /// ```
    pub fn from_bytes(v: &[u8]) -> Result<Self, LocaleError> {
        Ok(parser::parse_locale(v)?)
    }

    /// A constructor which takes optional subtags as `AsRef<[u8]>`, parses them and
    /// produces a well-formed `Locale`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let loc = Locale::from_parts("fr".parse().unwrap(), None, Some("CA".parse().unwrap()), &[], None);
    ///
    /// assert_eq!(loc.to_string(), "fr-CA");
    /// ```
    pub fn from_parts(
        language: subtags::Language,
        script: Option<subtags::Script>,
        region: Option<subtags::Region>,
        variants: &[subtags::Variant],
        extensions: Option<extensions::ExtensionsMap>,
    ) -> Self {
        let id = LanguageIdentifier::from_parts(language, script, region, variants);
        Locale {
            id,
            extensions: extensions.unwrap_or_default(),
        }
    }

    /// # Safety
    ///
    /// This function accepts subtags expecting variants
    /// to be deduplicated and ordered.
    pub const unsafe fn from_raw_parts_unchecked(
        language: subtags::Language,
        script: Option<subtags::Script>,
        region: Option<subtags::Region>,
        variants: Option<Box<[subtags::Variant]>>,
        extensions: extensions::ExtensionsMap,
    ) -> Self {
        let id = LanguageIdentifier::from_raw_parts_unchecked(language, script, region, variants);
        Self { id, extensions }
    }

    /// Consumes `Locale` and produces raw internal representations
    /// of all subtags in form of `u64`/`u32`.
    ///
    /// Primarily used for storing internal representation and restoring via
    /// `from_raw_parts_unchecked`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    /// use tinystr::{TinyStr8, TinyStr4};
    ///
    /// let loc: Locale = "en-US".parse()
    ///     .expect("Parsing failed.");
    ///
    /// let (lang, script, region, variants, extensions) = loc.into_parts();
    ///
    /// let loc2 = Locale::from_parts(
    ///     lang,
    ///     script,
    ///     region,
    ///     &variants,
    ///     Some(extensions.parse().unwrap())
    /// );
    ///
    /// assert_eq!(loc2.to_string(), "en-US");
    /// ```
    pub fn into_parts(self) -> PartsTuple {
        let (lang, region, script, variants) = self.id.into_parts();
        (lang, region, script, variants, self.extensions.to_string())
    }

    /// Compares a `Locale` to another `AsRef<Locale`
    /// allowing for either side to use the missing fields as wildcards.
    ///
    /// This allows for matching between `en` (treated as `en-*-*-*`) and `en-US`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let loc1: Locale = "en".parse()
    ///     .expect("Parsing failed.");
    ///
    /// let loc2: Locale = "en-US".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_ne!(loc1, loc2); // "en" != "en-US"
    /// assert_ne!(loc1.to_string(), loc2.to_string()); // "en" != "en-US"
    ///
    /// assert_eq!(loc1.matches(&loc2, false, false), false); // "en" != "en-US"
    /// assert_eq!(loc1.matches(&loc2, true, false), true); // "en-*-*-*" == "en-US"
    /// assert_eq!(loc1.matches(&loc2, false, true), false); // "en" != "en-*-US-*"
    /// assert_eq!(loc1.matches(&loc2, true, true), true); // "en-*-*-*" == "en-*-US-*"
    /// ```
    pub fn matches<O: AsRef<Self>>(
        &self,
        other: &O,
        self_as_range: bool,
        other_as_range: bool,
    ) -> bool {
        let other = other.as_ref();
        if !self.extensions.private.is_empty() || !other.extensions.private.is_empty() {
            return false;
        }
        self.id.matches(&other.id, self_as_range, other_as_range)
    }
}

impl FromStr for Locale {
    type Err = LocaleError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Ok(parser::parse_locale(source)?)
    }
}

impl From<LanguageIdentifier> for Locale {
    fn from(id: LanguageIdentifier) -> Self {
        Locale {
            id,
            extensions: ExtensionsMap::default(),
        }
    }
}

impl From<Locale> for LanguageIdentifier {
    fn from(value: Locale) -> Self {
        value.id
    }
}

impl AsRef<LanguageIdentifier> for Locale {
    fn as_ref(&self) -> &LanguageIdentifier {
        &self.id
    }
}

impl AsRef<Locale> for Locale {
    #[inline(always)]
    fn as_ref(&self) -> &Locale {
        self
    }
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.id, self.extensions)
    }
}

/// This is a best-effort operation that performs all available levels of canonicalization.
///
/// At the moment the operation will normalize casing and the separator, but in the future
/// it may also validate and update from deprecated subtags to canonical ones.
///
/// # Examples
///
/// ```
/// use unic_locale_impl::canonicalize;
///
/// assert_eq!(canonicalize("pL_latn_pl-U-HC-H12"), Ok("pl-Latn-PL-u-hc-h12".to_string()));
/// ```
pub fn canonicalize<S: AsRef<[u8]>>(input: S) -> Result<String, LocaleError> {
    let locale = Locale::from_bytes(input.as_ref())?;
    Ok(locale.to_string())
}
