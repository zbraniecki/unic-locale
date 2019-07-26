use std::str::FromStr;

use crate::errors::LocaleError;
use crate::parser::{parse_extension_subtags, ParserError};
use std::collections::HashMap;

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

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum UnicodeExtensionKey {
    HourCycle,
    Calendar,
    Collation,
    Capitalized,
    NumericalSystem,
}

impl FromStr for UnicodeExtensionKey {
    type Err = LocaleError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        match source {
            "hc" => Ok(UnicodeExtensionKey::HourCycle),
            "ca" => Ok(UnicodeExtensionKey::Calendar),
            "co" => Ok(UnicodeExtensionKey::Collation),
            "ka" => Ok(UnicodeExtensionKey::Capitalized),
            "nu" => Ok(UnicodeExtensionKey::NumericalSystem),
            _ => Err(LocaleError::Unknown),
        }
    }
}

impl Into<&'static str> for UnicodeExtensionKey {
    fn into(self) -> &'static str {
        match self {
            UnicodeExtensionKey::HourCycle => "hc",
            UnicodeExtensionKey::Calendar => "ca",
            UnicodeExtensionKey::Collation => "co",
            UnicodeExtensionKey::Capitalized => "ka",
            UnicodeExtensionKey::NumericalSystem => "nu",
        }
    }
}

impl Into<&'static str> for &UnicodeExtensionKey {
    fn into(self) -> &'static str {
        match self {
            UnicodeExtensionKey::HourCycle => "hc",
            UnicodeExtensionKey::Calendar => "ca",
            UnicodeExtensionKey::Collation => "co",
            UnicodeExtensionKey::Capitalized => "ka",
            UnicodeExtensionKey::NumericalSystem => "nu",
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct ExtensionsMap {
    unicode: HashMap<UnicodeExtensionKey, Option<String>>,
    transform: HashMap<String, Option<String>>,
    private: HashMap<String, Option<String>>,
}

impl ExtensionsMap {
    pub fn get_unicode(&self) -> &HashMap<UnicodeExtensionKey, Option<String>> {
        &self.unicode
    }

    pub fn get_transform(&self) -> &HashMap<String, Option<String>> {
        &self.transform
    }

    pub fn get_private(&self) -> &HashMap<String, Option<String>> {
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
}

impl FromStr for ExtensionsMap {
    type Err = ParserError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        parse_extension_subtags(source)
    }
}

impl std::fmt::Display for ExtensionsMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut parts: Vec<&str> = vec![];

        if !self.unicode.is_empty() {
            parts.push(ExtensionType::Unicode.into());

            let mut keys = vec![];
            for (k, value) in &self.unicode {
                keys.push((k.into(), value));
            }

            keys.sort();

            for (k, v) in keys {
                parts.push(k);
                if let Some(v) = v {
                    parts.push(v);
                }
            }
        }

        if !self.transform.is_empty() {
            parts.push(ExtensionType::Transform.into());

            let mut keys = vec![];
            for (k, value) in &self.transform {
                keys.push((k, value));
            }

            keys.sort();

            for (k, v) in keys {
                parts.push(k);
                if let Some(v) = v {
                    parts.push(v);
                }
            }
        }

        if !self.private.is_empty() {
            parts.push(ExtensionType::Private.into());

            let mut keys = vec![];
            for (k, value) in &self.private {
                keys.push((k, value));
            }

            keys.sort();

            for (k, v) in keys {
                parts.push(k);
                if let Some(v) = v {
                    parts.push(v);
                }
            }
        }
        write!(f, "{}", parts.join("-"))
    }
}
