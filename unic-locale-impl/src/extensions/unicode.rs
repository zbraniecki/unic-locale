use crate::errors::LocaleError;
use crate::parser::errors::ParserError;

use std::collections::BTreeMap;
use std::fmt::Write;
use std::str::FromStr;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct UnicodeExtensionList {
    // XXX: Type is a Vec<String>
    keywords: BTreeMap<String, Option<String>>,
    attributes: Vec<String>,
}

impl UnicodeExtensionList {
    pub fn is_empty(&self) -> bool {
        self.keywords.is_empty() && self.attributes.is_empty()
    }

    pub fn set(&mut self, key: &str, value: Option<String>) -> Result<(), LocaleError> {
        if key.len() == 2 {
            self.keywords.insert(String::from(key), value);
        } else {
            self.attributes.push(String::from(key));
        }
        Ok(())
    }
}

impl std::fmt::Display for UnicodeExtensionList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        f.write_char('u')?;

        for (k, t) in &self.keywords {
            if let Some(t) = t {
                write!(f, "-{}-{}", k, t)?;
            } else {
                write!(f, "-{}", k)?;
            }
        }

        for attr in &self.attributes {
            write!(f, "-{}", attr)?;
        }
        Ok(())
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub enum UnicodeExtensionKey {
    Calendar,       // ca
    CurrencyFormat, // cf
    Collation,      // co
    Currency,       // cu
    Emoji,          // em
    FirstDay,       // fw
    HourCycle,      // hc
    LineBreak,      // lb
    LineBreakWork,  // lw
    Measurement,    // ms
    Numbering,      // nu
    Region,         // rg
    RegionSubdiv,   // sd
    SentenceBreak,  // ss
    TimeZone,       // tz
    CommonVariant,  // va
}

impl FromStr for UnicodeExtensionKey {
    type Err = ParserError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        match source {
            "ca" => Ok(UnicodeExtensionKey::Calendar),
            "co" => Ok(UnicodeExtensionKey::Collation),
            "hc" => Ok(UnicodeExtensionKey::HourCycle),
            _ => Err(ParserError::InvalidExtension),
        }
    }
}

impl std::fmt::Display for UnicodeExtensionKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            UnicodeExtensionKey::Calendar => "ca",
            UnicodeExtensionKey::Collation => "co",
            UnicodeExtensionKey::HourCycle => "hc",
            _ => unimplemented!(),
        };
        f.write_str(s)
    }
}
