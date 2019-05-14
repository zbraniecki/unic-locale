pub mod errors;
pub mod extensions;
pub mod parser;

use std::collections::HashMap;
use unic_langid::LanguageIdentifier;
use errors::LocaleError;
use extensions::ExtensionType;

#[derive(Debug, PartialEq)]
pub struct Locale {
    pub langid: LanguageIdentifier,
    pub extensions: HashMap<ExtensionType, HashMap<String, String>>,
}

impl Locale {
    pub fn new() -> Self {
        Locale {
            langid: LanguageIdentifier::new(),
            extensions: HashMap::new(),
        }
    }

    pub fn from_str(ident: &str) -> Result<Self, errors::LocaleError> {
        parser::parse_locale(ident).map_err(|err| err.into())
    }

    pub fn matches(&self, other: &Self, self_as_range: bool, other_as_range: bool) -> bool {
        if self.extensions.contains_key(&ExtensionType::Private) || other.extensions.contains_key(&ExtensionType::Private) {
            return false;
        }
        self.langid
            .matches(&other.langid, self_as_range, other_as_range)
    }

    pub fn get_language(&self) -> &str {
        self.langid.get_language()
    }

    pub fn set_language(&mut self, language: Option<&str>) -> Result<(), LocaleError> {
        self.langid.set_language(language)
            .map_err(std::convert::Into::into)
    }

    pub fn get_script(&self) -> &Option<String> {
        self.langid.get_script()
    }

    pub fn set_script(&mut self, script: Option<&str>) -> Result<(), LocaleError> {
        self.langid.set_script(script)
            .map_err(std::convert::Into::into)
    }

    pub fn get_region(&self) -> &Option<String> {
        self.langid.get_region()
    }

    pub fn set_region(&mut self, region: Option<&str>) -> Result<(), LocaleError> {
        self.langid.set_region(region)
            .map_err(std::convert::Into::into)
    }

    pub fn get_variants(&self) -> &[String] {
        self.langid.get_variants()
    }

    pub fn set_variants(&mut self, variants: &[&str]) -> Result<(), LocaleError> {
        self.langid.set_variants(variants)
            .map_err(std::convert::Into::into)
    }
}

pub fn serialize_locale(loc: &Locale) -> Result<String, errors::LocaleError> {
    let langtag = loc.langid.to_string();
    let mut subtags = vec![langtag.as_str()];
    for (name, ext) in &loc.extensions {
        subtags.push(&extensions::convert_ext_type_to_string(&name));

        for (key, value) in ext {
            subtags.push(&extensions::convert_key_to_ext_key(&key).unwrap());
            if value != "true" {
                subtags.push(&value);
            }
        }
    }

    Ok(subtags.join("-"))
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let result = serialize_locale(&self).unwrap();
        write!(f, "{}", result)
    }
}

pub fn canonicalize(input: &str) -> Result<String, LocaleError> {
    let locale = Locale::from_str(input)?;
    Ok(locale.to_string())
}
