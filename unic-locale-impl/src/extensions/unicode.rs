use crate::errors::LocaleError;
use crate::parser::ParserError;

use std::collections::BTreeMap;
use std::iter::Peekable;

use tinystr::{TinyStr4, TinyStr8};

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct UnicodeExtensionList {
    // Canonical: sort by key (BTreeMap is already) / remove value 'true'
    keywords: BTreeMap<TinyStr4, Vec<TinyStr8>>,

    // Canonical: sort / dudup
    attributes: Vec<TinyStr8>,
}

fn parse_key(key: &str) -> Result<TinyStr4, ParserError> {
    if key.len() != 2
        || !key.as_bytes()[0].is_ascii_alphanumeric()
        || !key.as_bytes()[1].is_ascii_alphabetic()
    {
        return Err(ParserError::InvalidSubtag);
    }
    let key: TinyStr4 = key.parse().map_err(|_| ParserError::InvalidSubtag)?;
    Ok(key.to_ascii_lowercase())
}

const TRUE_TYPE: TinyStr8 = unsafe{ TinyStr8::new_unchecked(1702195828u64) }; // "true"

fn parse_type(t: &str) -> Result<Option<TinyStr8>, ParserError> {
    let s: TinyStr8 = t.parse().map_err(|_| ParserError::InvalidSubtag)?;
    if t.len() < 3 || t.len() > 8 || !s.is_ascii_alphanumeric() {
        return Err(ParserError::InvalidSubtag);
    }

    let s = s.to_ascii_lowercase();

    if s == TRUE_TYPE {
        Ok(None)
    } else {
        Ok(Some(s))
    }
}

fn parse_attribute(t: &str) -> Result<TinyStr8, ParserError> {
    let s: TinyStr8 = t.parse().map_err(|_| ParserError::InvalidSubtag)?;
    if t.len() < 3 || t.len() > 8 || !s.is_ascii_alphanumeric() {
        return Err(ParserError::InvalidSubtag);
    }

    Ok(s.to_ascii_lowercase())
}

fn is_attribute(t: &str) -> bool {
    let slen = t.len();
    (slen >= 3 && slen <= 8) && !t.contains(|c: char| !c.is_ascii_alphanumeric())
}

fn is_type(t: &str) -> bool {
    let slen = t.len();
    (slen >= 3 && slen <= 8) && !t.contains(|c: char| !c.is_ascii_alphanumeric())
}

impl UnicodeExtensionList {
    pub fn is_empty(&self) -> bool {
        self.keywords.is_empty() && self.attributes.is_empty()
    }

    pub fn set_keyword(&mut self, key: &str, value: Vec<&str>) -> Result<(), LocaleError> {
        let key = parse_key(key)?;

        let mut t = Vec::with_capacity(value.len());
        for val in value {
            if let Some(ty) = parse_type(val)? {
                t.push(ty);
            }
        }

        self.keywords.insert(key, t);
        Ok(())
    }

    pub fn set_attribute(&mut self, value: &str) -> Result<(), LocaleError> {
        let value = parse_attribute(value)?;
        let idx = self.attributes.binary_search(&value);
        if idx.is_err() {
            self.attributes.insert(idx.unwrap_err(), value);
        }
        Ok(())
    }

    pub fn try_from_iter<'a>(
        iter: &mut Peekable<impl Iterator<Item = &'a str>>,
    ) -> Result<Self, ParserError> {
        let mut uext = Self::default();

        let mut st_peek = iter.peek();

        let mut current_keyword = None;
        let mut current_types = vec![];

        while let Some(subtag) = st_peek {
            let slen = subtag.len();
            if slen == 2 {
                if let Some(current_keyword) = current_keyword {
                    uext.keywords.insert(current_keyword, current_types);
                    current_types = vec![];
                }
                current_keyword = Some(parse_key(subtag)?);
                iter.next();
            } else if current_keyword.is_some() && is_type(subtag) {
                if let Some(ty) = parse_type(subtag)? {
                    current_types.push(ty);
                }
                iter.next();
            } else if is_attribute(subtag) {
                uext.attributes.push(parse_attribute(subtag)?);
                iter.next();
            } else {
                break;
            }
            st_peek = iter.peek();
        }

        if let Some(current_keyword) = current_keyword {
            uext.keywords.insert(current_keyword, current_types);
        }

        uext.attributes.sort();
        uext.attributes.dedup();

        Ok(uext)
    }
}

impl std::fmt::Display for UnicodeExtensionList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        f.write_str("-u")?;

        for attr in &self.attributes {
            write!(f, "-{}", attr)?;
        }

        for (k, t) in &self.keywords {
            write!(f, "-{}", k)?;
            for v in t {
                write!(f, "-{}", v)?;
            }
        }
        Ok(())
    }
}
