pub mod errors;
pub mod parser;
pub mod subtags;

use crate::errors::LanguageIdentifierError;

#[derive(Default, Debug)]
pub struct LanguageIdentifier {
    language: Option<String>,
    script: Option<String>,
    region: Option<String>,
    variants: Vec<String>,
}

impl LanguageIdentifier {
    pub fn new() -> Self {
        LanguageIdentifier {
            language: None,
            script: None,
            region: None,
            variants: vec![],
        }
    }

    pub fn from_str(ident: &str) -> Result<Self, LanguageIdentifierError> {
        parser::parse_language_identifier(ident).map_err(std::convert::Into::into)
    }

    pub fn from_parts(
        language: Option<&str>,
        script: Option<&str>,
        region: Option<&str>,
        variants: &[&str],
    ) -> Result<Self, LanguageIdentifierError> {
        let language = if let Some(subtag) = language {
            subtags::parse_language_subtag(subtag)?
        } else {
            None
        };
        let script = if let Some(subtag) = script {
            Some(subtags::parse_script_subtag(subtag)?)
        } else {
            None
        };
        let region = if let Some(subtag) = region {
            Some(subtags::parse_region_subtag(subtag)?)
        } else {
            None
        };
        let mut variants_field = vec![];

        for variant in variants {
            variants_field.push(subtags::parse_variant_subtag(variant)?);
        }
        Ok(Self {
            language,
            script,
            region,
            variants: variants_field,
        })
    }

    pub fn matches(&self, other: &Self, self_as_range: bool, other_as_range: bool) -> bool {
        subtag_matches(
            &self.language,
            &other.language,
            self_as_range,
            other_as_range,
        ) && subtag_matches(&self.script, &other.script, self_as_range, other_as_range)
            && subtag_matches(&self.region, &other.region, self_as_range, other_as_range)
            && subtags_match(
                &self.variants,
                &other.variants,
                self_as_range,
                other_as_range,
            )
    }

    pub fn get_language(&self) -> &str {
        self.language.as_ref().map(String::as_str).unwrap_or("und")
    }

    pub fn set_language(&mut self, language: Option<&str>) -> Result<(), LanguageIdentifierError> {
        self.language = if let Some(lang) = language {
            subtags::parse_language_subtag(lang)?
        } else {
            None
        };
        Ok(())
    }

    pub fn get_script(&self) -> &Option<String> {
        &self.script
    }

    pub fn set_script(&mut self, script: Option<&str>) -> Result<(), LanguageIdentifierError> {
        self.script = if let Some(script) = script {
            Some(subtags::parse_script_subtag(script)?)
        } else {
            None
        };
        Ok(())
    }

    pub fn get_region(&self) -> &Option<String> {
        &self.region
    }

    pub fn set_region(&mut self, region: Option<&str>) -> Result<(), LanguageIdentifierError> {
        self.region = if let Some(region) = region {
            Some(subtags::parse_region_subtag(region)?)
        } else {
            None
        };
        Ok(())
    }

    pub fn get_variants(&self) -> &[String] {
        &self.variants
    }

    pub fn set_variants(&mut self, variants: &[&str]) -> Result<(), LanguageIdentifierError> {
        self.variants.clear();
        for variant in variants {
            self.variants.push(subtags::parse_variant_subtag(variant)?);
        }
        Ok(())
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
        for variant in &self.variants {
            subtags.push(variant);
        }

        write!(f, "{}", subtags.join("-"))
    }
}

impl std::cmp::PartialEq for LanguageIdentifier {
    fn eq(&self, other: &Self) -> bool {
        self.matches(other, false, false)
    }
}

fn subtag_matches(
    subtag1: &Option<String>,
    subtag2: &Option<String>,
    as_range1: bool,
    as_range2: bool,
) -> bool {
    (as_range1 && subtag1.is_none()) || (as_range2 && subtag2.is_none()) || subtag1 == subtag2
}

fn subtags_match(subtag1: &[String], subtag2: &[String], as_range1: bool, as_range2: bool) -> bool {
    (as_range1 && subtag1.is_empty()) || (as_range2 && subtag2.is_empty()) || subtag1 == subtag2
}
