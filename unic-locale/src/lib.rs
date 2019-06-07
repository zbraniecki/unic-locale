pub mod errors;
pub mod extensions;
pub mod parser;

use errors::LocaleError;
pub use extensions::ExtensionType;
use std::collections::HashMap;
use unic_langid::LanguageIdentifier;

#[derive(Debug, Default, PartialEq)]
pub struct Locale {
    pub langid: LanguageIdentifier,
    pub extensions: HashMap<ExtensionType, HashMap<String, String>>,
}

impl Locale {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_str(ident: &str) -> Result<Self, errors::LocaleError> {
        parser::parse_locale(ident).map_err(std::convert::Into::into)
    }

    pub fn from_str_with_options(ident: &str, options: HashMap<&str, &str>) -> Result<Self, errors::LocaleError> {
        let mut loc = parser::parse_locale(ident).map_err(std::convert::Into::into);
        if let Ok(ref mut loc) = loc {
            for (key, value) in options {
                if key == "language" {
                    loc.langid.set_language(Some(value))?;
                } else if key == "script" {
                    loc.langid.set_script(Some(value))?;
                } else if key == "region" {
                    loc.langid.set_region(Some(value))?;
                } else {
                    loc.set_extension(ExtensionType::Unicode, key, value)?;
                }
            }
        }
        loc
    }

    pub fn matches(&self, other: &Self, self_as_range: bool, other_as_range: bool) -> bool {
        if self.extensions.contains_key(&ExtensionType::Private)
            || other.extensions.contains_key(&ExtensionType::Private)
        {
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

    pub fn get_script(&self) -> &Option<String> {
        self.langid.get_script()
    }

    pub fn set_script(&mut self, script: Option<&str>) -> Result<(), LocaleError> {
        self.langid
            .set_script(script)
            .map_err(std::convert::Into::into)
    }

    pub fn get_region(&self) -> &Option<String> {
        self.langid.get_region()
    }

    pub fn set_region(&mut self, region: Option<&str>) -> Result<(), LocaleError> {
        self.langid
            .set_region(region)
            .map_err(std::convert::Into::into)
    }

    pub fn get_variants(&self) -> &[String] {
        self.langid.get_variants()
    }

    pub fn set_variants(&mut self, variants: &[&str]) -> Result<(), LocaleError> {
        self.langid
            .set_variants(variants)
            .map_err(std::convert::Into::into)
    }

    pub fn set_extension(
        &mut self,
        extension: ExtensionType,
        key: &str,
        value: &str,
    ) -> Result<(), LocaleError> {
        let ext = self
            .extensions
            .entry(extension)
            .or_insert(HashMap::new());
        //XXX: Check that the value is valid
        ext.insert(key.to_string(), value.to_string());
        Ok(())
    }
}

impl From<LanguageIdentifier> for Locale {
    fn from(langid: LanguageIdentifier) -> Self {
        Locale {
            langid,
            extensions: HashMap::new(),
        }
    }
}

impl Into<LanguageIdentifier> for Locale {
    fn into(self) -> LanguageIdentifier {
        self.langid
    }
}

pub fn serialize_locale(loc: &Locale) -> Result<String, errors::LocaleError> {
    let langtag = loc.langid.to_string();
    let mut subtags = vec![langtag.as_str()];
    for (name, ext) in &loc.extensions {
        subtags.push(&extensions::convert_ext_type_to_string(&name));

        let mut keys: Vec<&String> = ext.keys().collect();
        keys.sort();
        for key in keys {
            subtags.push(&extensions::convert_key_to_ext_key(&key).unwrap());
            if let Some(value) = ext.get(key) {
                if value != "true" {
                    subtags.push(&value);
                }
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
