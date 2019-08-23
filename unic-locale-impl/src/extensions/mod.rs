mod private;
mod transform;
mod unicode;

pub use private::PrivateExtensionList;
pub use transform::TransformExtensionList;
pub use unicode::UnicodeExtensionList;

use std::collections::BTreeMap;
use std::fmt::Write;
use std::iter::Peekable;
use std::str::FromStr;

use tinystr::TinyStr8;

use crate::parser::ParserError;

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
    pub other: BTreeMap<char, Vec<TinyStr8>>,
    pub private: PrivateExtensionList,
}

impl ExtensionsMap {
    pub fn try_from_iter<'a>(
        iter: &mut Peekable<impl Iterator<Item = &'a str>>,
    ) -> Result<Self, ParserError> {
        let mut result = ExtensionsMap::default();

        let mut st = iter.next();
        while let Some(subtag) = st {
            let subtag = subtag.to_ascii_lowercase();

            match subtag.as_str() {
                "" => break,
                "u" => {
                    result.unicode = UnicodeExtensionList::try_from_iter(iter)?;
                }
                "t" => {
                    result.transform = TransformExtensionList::try_from_iter(iter)?;
                }
                "x" => {
                    result.private = PrivateExtensionList::try_from_iter(iter)?;
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
        let mut iterator = source.split(|c| SEPARATORS.contains(&c)).peekable();
        Self::try_from_iter(&mut iterator)
    }
}

impl std::fmt::Display for ExtensionsMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}{}", self.unicode, self.transform, self.private)?;

        Ok(())
    }
}
