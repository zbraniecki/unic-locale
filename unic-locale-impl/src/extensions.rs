use std::str::FromStr;

use crate::errors::LocaleError;
use crate::parser::{parse_extension_subtags, ParserError};
use std::collections::BTreeMap;
use std::fmt::Write;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum ExtensionType {
    Unicode,
    Transform,
    Private,
}

impl FromStr for ExtensionType {
    type Err = LocaleError;

    fn from_str(key: &str) -> Result<Self, Self::Err> {
        match key {
            "u" => Ok(ExtensionType::Unicode),
            "t" => Ok(ExtensionType::Transform),
            "x" => Ok(ExtensionType::Private),
            _ => Err(LocaleError::Unknown),
        }
    }
}

impl std::fmt::Display for ExtensionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let ch = match self {
            ExtensionType::Unicode => 'u',
            ExtensionType::Transform => 't',
            ExtensionType::Private => 'x',
        };
        f.write_char(ch)
    }
}

impl Into<&'static str> for ExtensionType {
    fn into(self) -> &'static str {
        match self {
            ExtensionType::Unicode => "u",
            ExtensionType::Transform => "t",
            ExtensionType::Private => "x",
        }
    }
}

impl Into<&'static str> for &ExtensionType {
    fn into(self) -> &'static str {
        match self {
            ExtensionType::Unicode => "u",
            ExtensionType::Transform => "t",
            ExtensionType::Private => "x",
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub enum UnicodeExtensionKey {
    Calendar,
    Collation,
    HourCycle,
    Capitalized,
    NumericalSystem,
}

impl FromStr for UnicodeExtensionKey {
    type Err = LocaleError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        match source {
            "ca" => Ok(UnicodeExtensionKey::Calendar),
            "co" => Ok(UnicodeExtensionKey::Collation),
            "hc" => Ok(UnicodeExtensionKey::HourCycle),
            "ka" => Ok(UnicodeExtensionKey::Capitalized),
            "nu" => Ok(UnicodeExtensionKey::NumericalSystem),
            _ => Err(LocaleError::Unknown),
        }
    }
}

impl std::fmt::Display for UnicodeExtensionKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            UnicodeExtensionKey::Calendar => "ca",
            UnicodeExtensionKey::Collation => "co",
            UnicodeExtensionKey::HourCycle => "hc",
            UnicodeExtensionKey::Capitalized => "ka",
            UnicodeExtensionKey::NumericalSystem => "nu",
        };
        f.write_str(s)
    }
}

impl Into<&'static str> for UnicodeExtensionKey {
    fn into(self) -> &'static str {
        match self {
            UnicodeExtensionKey::Calendar => "ca",
            UnicodeExtensionKey::Collation => "co",
            UnicodeExtensionKey::HourCycle => "hc",
            UnicodeExtensionKey::Capitalized => "ka",
            UnicodeExtensionKey::NumericalSystem => "nu",
        }
    }
}

impl Into<&'static str> for &UnicodeExtensionKey {
    fn into(self) -> &'static str {
        match self {
            UnicodeExtensionKey::Calendar => "ca",
            UnicodeExtensionKey::Collation => "co",
            UnicodeExtensionKey::HourCycle => "hc",
            UnicodeExtensionKey::Capitalized => "ka",
            UnicodeExtensionKey::NumericalSystem => "nu",
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct ExtensionsMap {
    unicode: BTreeMap<UnicodeExtensionKey, Option<String>>,
    transform: BTreeMap<String, Option<String>>,
    private: BTreeMap<String, Option<String>>,
}

impl ExtensionsMap {
    pub fn get_unicode(&self) -> &BTreeMap<UnicodeExtensionKey, Option<String>> {
        &self.unicode
    }

    pub fn get_transform(&self) -> &BTreeMap<String, Option<String>> {
        &self.transform
    }

    pub fn get_private(&self) -> &BTreeMap<String, Option<String>> {
        &self.private
    }

    pub fn set_unicode_value(
        &mut self,
        key: UnicodeExtensionKey,
        value: Option<&str>,
    ) -> Result<(), LocaleError> {
        //XXX: Validate value
        self.unicode.insert(key, value.map(String::from));
        Ok(())
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
        if !self.unicode.is_empty() {
            write!(f, "{}", ExtensionType::Unicode)?;

            for (key, value) in &self.unicode {
                if let Some(value) = value {
                    write!(f, "-{}-{}", key, value)?;
                } else {
                    write!(f, "-{}", key)?;
                }
            }
        }

        if !self.transform.is_empty() {
            write!(f, "{}", ExtensionType::Transform)?;

            for (key, value) in &self.transform {
                f.write_char('-')?;
                f.write_str(key)?;
                if let Some(value) = value {
                    f.write_char('-')?;
                    f.write_str(value)?;
                }
            }
        }

        if !self.private.is_empty() {
            write!(f, "{}", ExtensionType::Private)?;

            for (key, value) in &self.private {
                f.write_char('-')?;
                f.write_str(key)?;
                if let Some(value) = value {
                    f.write_char('-')?;
                    f.write_str(value)?;
                }
            }
        }
        Ok(())
    }
}
