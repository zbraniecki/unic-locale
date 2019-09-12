pub mod errors;
pub mod parser;
pub mod subtags;

pub use crate::errors::LanguageIdentifierError;
use std::iter::Peekable;
use std::str::FromStr;

use tinystr::{TinyStr4, TinyStr8};

pub trait LangId {
    fn get_language(&self) -> &str;
    fn set_language(&mut self, language: Option<&str>) -> Result<(), LanguageIdentifierError>;

    fn get_script(&self) -> Option<&str>;
    fn set_script(&mut self, script: Option<&str>) -> Result<(), LanguageIdentifierError>;

    fn get_region(&self) -> Option<&str>;
    fn set_region(&mut self, region: Option<&str>) -> Result<(), LanguageIdentifierError>;

    fn get_variants(&self) -> Vec<&str>;
    fn set_variants(&mut self, variants: &[&str]) -> Result<(), LanguageIdentifierError>;

    fn matches<O: LangId>(&self, other: &O, self_as_range: bool, other_as_range: bool) -> bool;
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Hash)]
pub struct LanguageIdentifier {
    language: Option<TinyStr8>,
    script: Option<TinyStr4>,
    region: Option<TinyStr4>,
    // We store it as an Option to allow for const constructor.
    // Once const constructor for Box::new stabilizes, we can remove this.
    variants: Option<Box<[TinyStr8]>>,
}

impl LanguageIdentifier {
    pub fn from_parts<S: AsRef<str>>(
        language: Option<S>,
        script: Option<S>,
        region: Option<S>,
        variants: &[S],
    ) -> Result<Self, LanguageIdentifierError> {
        let language = if let Some(subtag) = language {
            subtags::parse_language_subtag(subtag.as_ref())?
        } else {
            None
        };
        let script = if let Some(subtag) = script {
            Some(subtags::parse_script_subtag(subtag.as_ref())?)
        } else {
            None
        };
        let region = if let Some(subtag) = region {
            Some(subtags::parse_region_subtag(subtag.as_ref())?)
        } else {
            None
        };

        let variants = if !variants.is_empty() {
            let mut vars = variants
                .into_iter()
                .map(|v| subtags::parse_variant_subtag(v.as_ref()))
                .collect::<Result<Vec<TinyStr8>, parser::errors::ParserError>>()?;
            vars.sort();
            vars.dedup();
            Some(vars.into_boxed_slice())
        } else {
            None
        };

        Ok(Self {
            language,
            script,
            region,
            variants,
        })
    }

    pub fn try_from_iter<'a>(
        iter: &mut Peekable<impl Iterator<Item = &'a str>>,
        allow_extension: bool,
    ) -> Result<LanguageIdentifier, LanguageIdentifierError> {
        parser::parse_language_identifier_from_iter(iter, allow_extension)
            .map_err(std::convert::Into::into)
    }

    pub fn into_raw_parts(self) -> (Option<u64>, Option<u32>, Option<u32>, Option<Box<[u64]>>) {
        (
            self.language.map(|l| l.into()),
            self.script.map(|s| s.into()),
            self.region.map(|r| r.into()),
            self.variants
                .map(|v| v.iter().map(|v| (*v).into()).collect()),
        )
    }

    #[inline(always)]
    pub const unsafe fn from_raw_parts_unchecked(
        language: Option<TinyStr8>,
        script: Option<TinyStr4>,
        region: Option<TinyStr4>,
        variants: Option<Box<[TinyStr8]>>,
    ) -> Self {
        Self {
            language,
            script,
            region,
            variants,
        }
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

    pub fn matches<O: LangId>(&self, other: &O, self_as_range: bool, other_as_range: bool) -> bool {
        LangId::matches(self, other, self_as_range, other_as_range)
    }
}

impl LangId for LanguageIdentifier {
    fn get_language(&self) -> &str {
        self.language.as_ref().map(|s| s.as_ref()).unwrap_or("und")
    }

    fn set_language(&mut self, language: Option<&str>) -> Result<(), LanguageIdentifierError> {
        self.language = if let Some(lang) = language {
            subtags::parse_language_subtag(lang)?
        } else {
            None
        };
        Ok(())
    }

    fn get_script(&self) -> Option<&str> {
        self.script.as_ref().map(|s| s.as_ref())
    }

    fn set_script(&mut self, script: Option<&str>) -> Result<(), LanguageIdentifierError> {
        self.script = if let Some(script) = script {
            Some(subtags::parse_script_subtag(script)?)
        } else {
            None
        };
        Ok(())
    }

    fn get_region(&self) -> Option<&str> {
        self.region.as_ref().map(|s| s.as_ref())
    }

    fn set_region(&mut self, region: Option<&str>) -> Result<(), LanguageIdentifierError> {
        self.region = if let Some(region) = region {
            Some(subtags::parse_region_subtag(region)?)
        } else {
            None
        };
        Ok(())
    }

    fn get_variants(&self) -> Vec<&str> {
        if let Some(variants) = &self.variants {
            variants.iter().map(|s| s.as_ref()).collect()
        } else {
            vec![]
        }
    }

    fn set_variants(&mut self, variants: &[&str]) -> Result<(), LanguageIdentifierError> {
        if variants.is_empty() {
            self.variants = None;
        } else {
            let mut result = variants
                .into_iter()
                .map(|v| subtags::parse_variant_subtag(v.as_ref()))
                .collect::<Result<Vec<TinyStr8>, parser::errors::ParserError>>()?;
            result.sort();
            result.dedup();
            self.variants = Some(result.into_boxed_slice());
        }
        Ok(())
    }

    fn matches<O: LangId>(&self, other: &O, self_as_range: bool, other_as_range: bool) -> bool {
        lang_matches(
            self.get_language(),
            other.get_language(),
            self_as_range,
            other_as_range,
        ) && subtag_matches(
            self.get_script(),
            other.get_script(),
            self_as_range,
            other_as_range,
        ) && subtag_matches(
            self.get_region(),
            other.get_region(),
            self_as_range,
            other_as_range,
        ) && subtags_match(
            &self.get_variants(),
            &other.get_variants(),
            self_as_range,
            other_as_range,
        )
    }
}

impl FromStr for LanguageIdentifier {
    type Err = LanguageIdentifierError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        parser::parse_language_identifier(source).map_err(std::convert::Into::into)
    }
}

impl Into<LanguageIdentifier> for &LanguageIdentifier {
    fn into(self) -> LanguageIdentifier {
        self.clone()
    }
}

impl AsRef<LanguageIdentifier> for LanguageIdentifier {
    #[inline(always)]
    fn as_ref(&self) -> &LanguageIdentifier {
        self
    }
}

impl std::fmt::Display for LanguageIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut subtags = vec![self.get_language()];
        if let Some(script) = self.get_script() {
            subtags.push(script);
        }
        if let Some(region) = self.get_region() {
            subtags.push(region);
        }
        if let Some(variants) = &self.variants {
            for variant in variants.iter() {
                subtags.push(variant);
            }
        }

        f.write_str(&subtags.join("-"))
    }
}

fn lang_matches(lang1: &str, lang2: &str, as_range1: bool, as_range2: bool) -> bool {
    (lang1 == "und" && as_range1) || (lang2 == "und" && as_range2) || lang1 == lang2
}

fn subtag_matches(
    subtag1: Option<&str>,
    subtag2: Option<&str>,
    as_range1: bool,
    as_range2: bool,
) -> bool {
    (subtag1.is_none() && as_range1) || (subtag2.is_none() && as_range2) || subtag1 == subtag2
}

fn subtags_match(subtag1: &[&str], subtag2: &[&str], as_range1: bool, as_range2: bool) -> bool {
    (subtag1.is_empty() && as_range1) || (subtag2.is_empty() && as_range2) || subtag1 == subtag2
}

pub fn canonicalize(input: &str) -> Result<String, LanguageIdentifierError> {
    let lang_id: LanguageIdentifier = input.parse()?;
    Ok(lang_id.to_string())
}
