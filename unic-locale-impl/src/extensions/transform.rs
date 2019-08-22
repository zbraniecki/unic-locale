use crate::parser::ParserError;

use unic_langid_impl::LanguageIdentifier;

use std::collections::BTreeMap;
use std::fmt::Write;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct TransformExtensionList {
    tlang: Option<LanguageIdentifier>,
    tfields: BTreeMap<String, Vec<String>>,
}

impl TransformExtensionList {
    pub fn is_empty(&self) -> bool {
        self.tlang.is_none() && self.tfields.is_empty()
    }

    pub fn parse_from_iter<'a>(
        iter: &mut impl Iterator<Item = &'a str>,
    ) -> Result<Self, ParserError> {
        let mut text = Self::default();

        let mut iter = iter.peekable();
        let mut st = iter.peek();

        while let Some(subtag) = st {
            let slen = subtag.len();

            if slen == 2
                && subtag.as_bytes()[0].is_ascii_alphabetic()
                && subtag.as_bytes()[1].is_ascii_digit()
            {

            } else {
                text.tlang = Some(
                    LanguageIdentifier::from_iter(&mut iter, true)
                        .map_err(|_| ParserError::InvalidLanguage)?,
                );
            }
            iter.next();
            st = iter.peek();
        }

        Ok(text)
    }
}

impl std::fmt::Display for TransformExtensionList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        f.write_char('t')?;

        if let Some(tlang) = &self.tlang {
            write!(f, "-{}", tlang)?;
        }

        for (k, t) in &self.tfields {
            if t.is_empty() {
                write!(f, "-{}", k)?;
            } else {
                write!(f, "-{}-{}", k, t.join("-"))?;
            }
        }
        Ok(())
    }
}
