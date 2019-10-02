pub mod errors;
pub mod extensions;
pub mod parser;

use errors::LocaleError;
pub use extensions::{ExtensionType, ExtensionsMap};
use std::str::FromStr;
use tinystr::{TinyStr4, TinyStr8};
pub use unic_langid_impl::CharacterDirection;
pub use unic_langid_impl::LanguageIdentifier;

#[derive(Debug, Default, PartialEq, Clone)]
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
    pub fn from_parts<S: AsRef<str>>(
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

    pub fn into_raw_parts(self) -> RawPartsTuple {
        let (lang, region, script, variants) = self.langid.into_raw_parts();
        (lang, region, script, variants, self.extensions.to_string())
    }

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

    pub fn get_language(&self) -> &str {
        self.langid.get_language()
    }

    pub fn set_language(&mut self, language: Option<&str>) -> Result<(), LocaleError> {
        self.langid
            .set_language(language)
            .map_err(std::convert::Into::into)
    }

    pub fn get_script(&self) -> Option<&str> {
        self.langid.get_script()
    }

    pub fn set_script(&mut self, script: Option<&str>) -> Result<(), LocaleError> {
        self.langid
            .set_script(script)
            .map_err(std::convert::Into::into)
    }

    pub fn get_region(&self) -> Option<&str> {
        self.langid.get_region()
    }

    pub fn set_region(&mut self, region: Option<&str>) -> Result<(), LocaleError> {
        self.langid
            .set_region(region)
            .map_err(std::convert::Into::into)
    }

    pub fn get_variants(&self) -> Vec<&str> {
        self.langid.get_variants()
    }

    pub fn set_variants(&mut self, variants: &[&str]) -> Result<(), LocaleError> {
        self.langid
            .set_variants(variants)
            .map_err(std::convert::Into::into)
    }

    #[cfg(feature = "likelysubtags")]
    pub fn add_likely_subtags(&mut self) -> bool {
        self.langid.add_likely_subtags()
    }

    #[cfg(feature = "likelysubtags")]
    pub fn remove_likely_subtags(&mut self) -> bool {
        self.langid.remove_likely_subtags()
    }

    pub fn get_character_direction(&self) -> CharacterDirection {
        self.langid.get_character_direction()
    }
}

impl FromStr for Locale {
    type Err = LocaleError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        parser::parse_locale(source).map_err(std::convert::Into::into)
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

pub fn canonicalize(input: &str) -> Result<String, LocaleError> {
    let locale: Locale = input.parse()?;
    Ok(locale.to_string())
}
