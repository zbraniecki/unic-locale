pub(crate) mod errors;
pub mod extensions;
pub mod parser;

use errors::LocaleError;
pub use extensions::{ExtensionType, ExtensionsMap};
use std::str::FromStr;
use tinystr::{TinyStr4, TinyStr8};
pub use unic_langid_impl::CharacterDirection;
pub use unic_langid_impl::LanguageIdentifier;

/// `Locale` is a core struct representing a Unicode Locale Identifier.
///
/// A locale is made of two parts:
///  * `langid` - Unicode Language Identifier
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
/// assert_eq!(loc.language(), "en");
/// assert_eq!(loc.script(), None);
/// assert_eq!(loc.region(), Some("US"));
/// assert_eq!(loc.variants().len(), 0);
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
/// assert_eq!(loc.language(), "en");
/// assert_eq!(loc.script(), Some("Latn"));
/// assert_eq!(loc.region(), Some("US"));
/// assert_eq!(loc.variants().collect::<Vec<_>>(), &["valencia"]);
/// ```
#[derive(Debug, Default, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Locale {
    pub langid: LanguageIdentifier,
    pub extensions: extensions::ExtensionsMap,
}

type RawPartsTuple = (
    Option<u64>,
    Option<u32>,
    Option<u32>,
    Option<Box<[u64]>>,
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
    /// let loc = Locale::from_parts(Some("fr"), None, Some("CA"), &[], None)
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.to_string(), "fr-CA");
    /// ```
    pub fn from_parts<S: AsRef<[u8]>>(
        language: Option<S>,
        script: Option<S>,
        region: Option<S>,
        variants: &[S],
        extensions: Option<extensions::ExtensionsMap>,
    ) -> Result<Self, LocaleError> {
        let langid = LanguageIdentifier::from_parts(language, script, region, variants)?;
        Ok(Locale {
            langid,
            extensions: extensions.unwrap_or_default(),
        })
    }

    /// Consumes `Locale` and produces raw internal representations
    /// of all subtags in form of `u64`/`u32`.
    ///
    /// Primarily used for storing internal representation and restoring via
    /// an unsafe `from_raw_parts_unchecked`.
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
    /// let (lang, script, region, variants, extensions) = loc.into_raw_parts();
    ///
    /// let loc2 = unsafe { Locale::from_raw_parts_unchecked(
    ///     lang.map(|l| TinyStr8::new_unchecked(l)),
    ///     script.map(|s| TinyStr4::new_unchecked(s)),
    ///     region.map(|r| TinyStr4::new_unchecked(r)),
    ///     variants.map(|v| v.into_iter().map(|v| TinyStr8::new_unchecked(*v)).collect()),
    ///     extensions.parse().unwrap()
    /// ) };
    ///
    /// assert_eq!(loc2.to_string(), "en-US");
    /// ```
    pub fn into_raw_parts(self) -> RawPartsTuple {
        let (lang, region, script, variants) = self.langid.into_raw_parts();
        (lang, region, script, variants, self.extensions.to_string())
    }

    /// Consumes raw representation of subtags generating new `Locale`
    /// without any checks.
    ///
    /// Primarily used for restoring internal representation.
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
    /// let (lang, script, region, variants, extensions) = loc.into_raw_parts();
    ///
    /// let loc2 = unsafe { Locale::from_raw_parts_unchecked(
    ///     lang.map(|l| TinyStr8::new_unchecked(l)),
    ///     script.map(|s| TinyStr4::new_unchecked(s)),
    ///     region.map(|r| TinyStr4::new_unchecked(r)),
    ///     variants.map(|v| v.into_iter().map(|v| TinyStr8::new_unchecked(*v)).collect()),
    ///     extensions.parse().unwrap()
    /// ) };
    ///
    /// assert_eq!(loc2.to_string(), "en-US");
    /// ```
    #[inline(always)]
    pub unsafe fn from_raw_parts_unchecked(
        language: Option<TinyStr8>,
        script: Option<TinyStr4>,
        region: Option<TinyStr4>,
        variants: Option<Box<[TinyStr8]>>,
        extensions: extensions::ExtensionsMap,
    ) -> Self {
        let langid =
            LanguageIdentifier::from_raw_parts_unchecked(language, script, region, variants);
        Self { langid, extensions }
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
        self.langid
            .matches(&other.langid, self_as_range, other_as_range)
    }

    /// Returns the language subtag of the `Locale`.
    ///
    /// If the language is empty, `"und"` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let loc1: Locale = "de-AT".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc1.language(), "de");
    ///
    /// let loc2: Locale = "und-AT".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc2.language(), "und");
    /// ```
    pub fn language(&self) -> &str {
        self.langid.language()
    }

    /// Sets the language subtag of the `Locale`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "de-Latn-AT".parse()
    ///     .expect("Parsing failed.");
    ///
    /// loc.set_language("fr")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.to_string(), "fr-Latn-AT");
    /// ```
    pub fn set_language<S: AsRef<[u8]>>(&mut self, language: S) -> Result<(), LocaleError> {
        Ok(self.langid.set_language(language)?)
    }

    /// Clears the language subtag of the `Locale`.
    ///
    /// An empty language subtag is serialized to `und`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "de-Latn-AT".parse()
    ///     .expect("Parsing failed.");
    ///
    /// loc.clear_language();
    ///
    /// assert_eq!(loc.to_string(), "und-Latn-AT");
    /// ```
    pub fn clear_language(&mut self) {
        self.langid.clear_language()
    }

    /// Returns the script subtag of the `Locale`, if set.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let loc1: Locale = "de-Latn-AT".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc1.script(), Some("Latn"));
    ///
    /// let loc2: Locale = "de-AT".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc2.script(), None);
    /// ```
    pub fn script(&self) -> Option<&str> {
        self.langid.script()
    }

    /// Sets the script subtag of the `Locale`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "sr-Latn".parse()
    ///     .expect("Parsing failed.");
    ///
    /// loc.set_script("Cyrl")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.to_string(), "sr-Cyrl");
    /// ```
    pub fn set_script<S: AsRef<[u8]>>(&mut self, script: S) -> Result<(), LocaleError> {
        Ok(self.langid.set_script(script)?)
    }

    /// Clears the script subtag of the `Locale`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "sr-Latn".parse()
    ///     .expect("Parsing failed.");
    ///
    /// loc.clear_script();
    ///
    /// assert_eq!(loc.to_string(), "sr");
    /// ```
    pub fn clear_script(&mut self) {
        self.langid.clear_script()
    }

    /// Returns the region subtag of the `Locale`, if set.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let loc1: Locale = "de-Latn-AT".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc1.region(), Some("AT"));
    ///
    /// let loc2: Locale = "de".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc2.region(), None);
    /// ```
    pub fn region(&self) -> Option<&str> {
        self.langid.region()
    }

    /// Sets the region subtag of the `Locale`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "fr-FR".parse()
    ///     .expect("Parsing failed.");
    ///
    /// loc.set_region("CA")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.to_string(), "fr-CA");
    /// ```
    pub fn set_region<S: AsRef<[u8]>>(&mut self, region: S) -> Result<(), LocaleError> {
        Ok(self.langid.set_region(region)?)
    }

    /// Clears the region subtag of the `Locale`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "fr-FR".parse()
    ///     .expect("Parsing failed.");
    ///
    /// loc.clear_region();
    ///
    /// assert_eq!(loc.to_string(), "fr");
    /// ```
    pub fn clear_region(&mut self) {
        self.langid.clear_region()
    }

    /// Returns a vector of variants subtags of the `Locale`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let loc1: Locale = "ca-ES-valencia".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc1.variants().collect::<Vec<_>>(), &["valencia"]);
    ///
    /// let loc2: Locale = "de".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc2.variants().len(), 0);
    /// ```
    pub fn variants(&self) -> impl ExactSizeIterator<Item = &str> {
        self.langid.variants()
    }

    /// Sets variant subtags of the `Locale`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "ca-ES".parse()
    ///     .expect("Parsing failed.");
    ///
    /// loc.set_variants(&["valencia"])
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.to_string(), "ca-ES-valencia");
    /// ```
    pub fn set_variants<S: AsRef<[u8]>>(
        &mut self,
        variants: impl IntoIterator<Item = S>,
    ) -> Result<(), LocaleError> {
        Ok(self.langid.set_variants(variants)?)
    }

    /// Tests if a variant subtag is present in the `Locale`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "ca-ES-macos".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.has_variant("valencia"), Ok(false));
    /// assert_eq!(loc.has_variant("macos"), Ok(true));
    /// ```
    pub fn has_variant<S: AsRef<[u8]>>(&self, variant: S) -> Result<bool, LocaleError> {
        Ok(self.langid.has_variant(variant)?)
    }

    /// Clears variant subtags of the `Locale`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "ca-ES-valencia".parse()
    ///     .expect("Parsing failed.");
    ///
    /// loc.clear_variants();
    ///
    /// assert_eq!(loc.to_string(), "ca-ES");
    /// ```
    pub fn clear_variants(&mut self) {
        self.langid.clear_variants()
    }

    /// Extends the `Locale` adding likely subtags based
    /// on tables provided by CLDR.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "en-US".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.maximize(), true);
    /// assert_eq!(loc.to_string(), "en-Latn-US");
    /// ```
    #[cfg(feature = "likelysubtags")]
    pub fn maximize(&mut self) -> bool {
        self.langid.maximize()
    }

    /// Extends the `Locale` removing likely subtags based
    /// on tables provided by CLDR.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::Locale;
    ///
    /// let mut loc: Locale = "en-Latn-US".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.minimize(), true);
    /// assert_eq!(loc.to_string(), "en");
    /// ```
    #[cfg(feature = "likelysubtags")]
    pub fn minimize(&mut self) -> bool {
        self.langid.minimize()
    }

    /// Returns character direction of the `Locale`.
    ///
    /// # Examples
    ///
    /// ```
    /// use unic_locale_impl::{Locale, CharacterDirection};
    ///
    /// let loc1: Locale = "es-AR".parse()
    ///     .expect("Parsing failed.");
    /// let loc2: Locale = "fa".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc1.character_direction(), CharacterDirection::LTR);
    /// assert_eq!(loc2.character_direction(), CharacterDirection::RTL);
    /// ```
    pub fn character_direction(&self) -> CharacterDirection {
        self.langid.character_direction()
    }
}

impl FromStr for Locale {
    type Err = LocaleError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Ok(parser::parse_locale(source)?)
    }
}

impl From<LanguageIdentifier> for Locale {
    fn from(langid: LanguageIdentifier) -> Self {
        Locale {
            langid,
            extensions: ExtensionsMap::default(),
        }
    }
}

impl Into<LanguageIdentifier> for Locale {
    fn into(self) -> LanguageIdentifier {
        self.langid
    }
}

impl AsRef<LanguageIdentifier> for Locale {
    fn as_ref(&self) -> &LanguageIdentifier {
        &self.langid
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
        write!(f, "{}{}", self.langid, self.extensions)
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
