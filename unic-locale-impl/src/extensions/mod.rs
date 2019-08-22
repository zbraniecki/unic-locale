mod private;
mod transform;
mod unicode;

pub use private::PrivateExtensionList;
pub use transform::TransformExtensionList;
pub use unicode::UnicodeExtensionList;

use std::str::FromStr;

use crate::parser::ParserError;
use std::collections::BTreeMap;
use std::fmt::Write;

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
            sign if sign.is_ascii_alphanumeric() => {
                Ok(ExtensionType::Other(sign.to_ascii_lowercase()))
            }
            _ => Err(ParserError::InvalidExtension),
        }
    }
}

impl std::fmt::Display for ExtensionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let ch = match self {
            ExtensionType::Unicode => 'u',
            ExtensionType::Transform => 't',
            ExtensionType::Other(n) => *n,
            ExtensionType::Private => 'x',
        };
        f.write_char(ch)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct ExtensionsMap {
    pub unicode: UnicodeExtensionList,
    pub transform: TransformExtensionList,
    other: BTreeMap<char, BTreeMap<String, Option<String>>>,
    pub private: PrivateExtensionList,
}

impl ExtensionsMap {
    pub fn from_iter<'a>(
        mut iter: &mut impl Iterator<Item = &'a str>,
    ) -> Result<Self, ParserError> {
        let mut result = ExtensionsMap::default();

        let mut st = iter.next();
        while let Some(subtag) = st {
            let subtag = subtag.to_ascii_lowercase();

            match subtag.as_str() {
                "" => break,
                "u" => {
                    result.unicode = UnicodeExtensionList::parse_from_iter(&mut iter)?;
                }
                "t" => {
                    result.transform = TransformExtensionList::parse_from_iter(&mut iter)?;
                }
                "x" => {
                    result.private = PrivateExtensionList::parse_from_iter(&mut iter)?;
                }
                _ => unimplemented!(),
            }

            st = iter.next();
        }

        Ok(result)
    }

    pub fn is_empty(&self) -> bool {
        self.unicode.is_empty() && self.transform.is_empty() && self.private.is_empty()
    }
}

static SEPARATORS: &[char] = &['-', '_'];

impl FromStr for ExtensionsMap {
    type Err = ParserError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let mut iterator = source.split(|c| SEPARATORS.contains(&c));
        Self::from_iter(&mut iterator)
    }
}

impl std::fmt::Display for ExtensionsMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.unicode)?;
        write!(f, "{}", self.transform)?;
        write!(f, "{}", self.private)?;

        Ok(())
    }
}
