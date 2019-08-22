use crate::errors::LocaleError;
use crate::parser::ParserError;

use std::collections::BTreeMap;
use std::fmt::Write;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct UnicodeExtensionList {
    keywords: BTreeMap<String, Vec<String>>,
    attributes: Vec<String>,
}

impl UnicodeExtensionList {
    pub fn is_empty(&self) -> bool {
        self.keywords.is_empty() && self.attributes.is_empty()
    }

    pub fn set_keyword(&mut self, key: &str, value: Vec<String>) -> Result<(), LocaleError> {
        self.keywords.insert(String::from(key), value);
        Ok(())
    }

    pub fn set_attribute(&mut self, value: String) -> Result<(), LocaleError> {
        self.attributes.push(value);
        Ok(())
    }

    pub fn parse_from_iter<'a>(
        iter: &mut impl Iterator<Item = &'a str>,
    ) -> Result<Self, ParserError> {
        let mut uext = Self::default();

        let mut st = iter.next();

        let mut current_keyword = None;
        let mut current_types = vec![];

        while let Some(subtag) = st {
            let slen = subtag.len();

            if slen == 2 {
                if let Some(current_keyword) = current_keyword {
                    uext.keywords.insert(current_keyword, current_types);
                    current_types = vec![];
                }
                current_keyword = Some(subtag.to_ascii_lowercase());
            } else if current_keyword.is_some() {
                current_types.push(subtag.to_ascii_lowercase());
            } else {
                uext.attributes.push(subtag.to_ascii_lowercase());
            }
            st = iter.next();
        }

        if let Some(current_keyword) = current_keyword {
            uext.keywords.insert(current_keyword, current_types);
        }

        Ok(uext)
    }
}

impl std::fmt::Display for UnicodeExtensionList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        f.write_char('u')?;

        for (k, t) in &self.keywords {
            if t.is_empty() {
                write!(f, "-{}", k)?;
            } else {
                write!(f, "-{}-{}", k, t.join("-"))?;
            }
        }

        for attr in &self.attributes {
            write!(f, "-{}", attr)?;
        }
        Ok(())
    }
}
