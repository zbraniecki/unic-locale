pub mod errors;
pub mod extensions;
pub mod parser;

use errors::LocaleError;
pub use extensions::{ExtensionType, ExtensionsMap};
use std::str::FromStr;
use tinystr::{TinyStr4, TinyStr8};
use unic_langid_impl::LangId;
pub use unic_langid_impl::LanguageIdentifier;
use unic_langid_impl::LanguageIdentifierError;

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

    pub fn get_language(&self) -> &str {
        LangId::get_language(self)
    }

    pub fn set_language(&mut self, language: Option<&str>) -> Result<(), LanguageIdentifierError> {
        LangId::set_language(self, language)
    }

    pub fn get_script(&self) -> Option<&str> {
        LangId::get_script(self)
    }

    pub fn set_script(&mut self, script: Option<&str>) -> Result<(), LanguageIdentifierError> {
        LangId::set_script(self, script)
    }

    pub fn get_region(&self) -> Option<&str> {
        LangId::get_region(self)
    }

    pub fn set_region(&mut self, region: Option<&str>) -> Result<(), LanguageIdentifierError> {
        LangId::set_region(self, region)
    }

    pub fn get_variants(&self) -> Vec<&str> {
        LangId::get_variants(self)
    }

    pub fn set_variants(&mut self, variants: &[&str]) -> Result<(), LanguageIdentifierError> {
        LangId::set_variants(self, variants)
    }

    pub fn matches(&self, other: &Locale, self_as_range: bool, other_as_range: bool) -> bool {
        if !self.extensions.private.is_empty() || !other.extensions.private.is_empty() {
            return false;
        }
        LangId::matches(self, other, self_as_range, other_as_range)
    }
}

impl LangId for Locale {
    fn matches<O: LangId>(&self, other: &O, self_as_range: bool, other_as_range: bool) -> bool {
        self.langid.matches(other, self_as_range, other_as_range)
    }

    fn get_language(&self) -> &str {
        self.langid.get_language()
    }

    fn set_language(&mut self, language: Option<&str>) -> Result<(), LanguageIdentifierError> {
        self.langid
            .set_language(language)
            .map_err(std::convert::Into::into)
    }

    fn get_script(&self) -> Option<&str> {
        self.langid.get_script()
    }

    fn set_script(&mut self, script: Option<&str>) -> Result<(), LanguageIdentifierError> {
        self.langid
            .set_script(script)
            .map_err(std::convert::Into::into)
    }

    fn get_region(&self) -> Option<&str> {
        self.langid.get_region()
    }

    fn set_region(&mut self, region: Option<&str>) -> Result<(), LanguageIdentifierError> {
        self.langid
            .set_region(region)
            .map_err(std::convert::Into::into)
    }

    fn get_variants(&self) -> Vec<&str> {
        self.langid.get_variants()
    }

    fn set_variants(&mut self, variants: &[&str]) -> Result<(), LanguageIdentifierError> {
        self.langid
            .set_variants(variants)
            .map_err(std::convert::Into::into)
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

impl Into<LanguageIdentifier> for &Locale {
    fn into(self) -> LanguageIdentifier {
        self.langid.clone()
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
