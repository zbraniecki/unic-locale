mod transform;
mod unicode;

pub use unicode::UnicodeExtensionList;

use std::str::FromStr;

use crate::errors::LocaleError;
use crate::parser::{parse_extension_subtags, ParserError};
use std::collections::BTreeMap;
use std::fmt::Write;

use tinystr::TinyStr4;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum ExtensionType {
    Unicode,
    Transform,
    Other(char),
    Private,
}

impl ExtensionType {
    pub fn from_char(key: char) -> Result<Self, ParserError> {
        match key {
            'u' => Ok(ExtensionType::Unicode),
            't' => Ok(ExtensionType::Transform),
            'x' => Ok(ExtensionType::Private),
            sign @ _ if sign.is_ascii_alphanumeric() => Ok(ExtensionType::Other(sign)),
            _ => Err(ParserError::InvalidExtension),
        }
    }
}

impl std::fmt::Display for ExtensionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let ch = match self {
            ExtensionType::Unicode => 'u',
            ExtensionType::Transform => 't',
            ExtensionType::Other(n) => n.clone(),
            ExtensionType::Private => 'x',
        };
        f.write_char(ch)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct ExtensionsMap {
    unicode: UnicodeExtensionList,
    transform: BTreeMap<String, Option<String>>,
    other: BTreeMap<char, BTreeMap<String, Option<String>>>,
    private: BTreeMap<String, Option<String>>,
}

impl ExtensionsMap {
    pub fn get_unicode(&self) -> &UnicodeExtensionList {
        &self.unicode
    }

    pub fn get_transform(&self) -> &BTreeMap<String, Option<String>> {
        &self.transform
    }

    pub fn get_private(&self) -> &BTreeMap<String, Option<String>> {
        &self.private
    }

    pub fn set_unicode_value(&mut self, key: &str, value: Option<&str>) -> Result<(), LocaleError> {
        self.unicode.set(key, value.map(|s| String::from(s)))
    }

    pub fn set_transform_value(
        &mut self,
        key: &str,
        value: Option<&str>,
    ) -> Result<(), LocaleError> {
        self.transform
            .insert(String::from(key), value.map(String::from));
        Ok(())
    }

    pub fn set_private_value(&mut self, key: &str, value: Option<&str>) -> Result<(), LocaleError> {
        self.private
            .insert(String::from(key), value.map(String::from));
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.unicode.is_empty() && self.transform.is_empty() && self.private.is_empty()
    }
}

impl FromStr for ExtensionsMap {
    type Err = ParserError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        parse_extension_subtags(source)
    }
}

impl std::fmt::Display for ExtensionsMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.unicode)?;

        if !self.transform.is_empty() {
            write!(f, "{}", ExtensionType::Transform)?;

            for (key, value) in &self.transform {
                if let Some(value) = value {
                    write!(f, "-{}-{}", key, value)?;
                } else {
                    write!(f, "-{}", key)?;
                }
            }
        }

        if !self.private.is_empty() {
            write!(f, "{}", ExtensionType::Private)?;

            for (key, value) in &self.private {
                if let Some(value) = value {
                    write!(f, "-{}-{}", key, value)?;
                } else {
                    write!(f, "-{}", key)?;
                }
            }
        }
        Ok(())
    }
}
