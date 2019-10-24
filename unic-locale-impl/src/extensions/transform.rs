use crate::errors::LocaleError;
use crate::parser::ParserError;

use unic_langid_impl::LanguageIdentifier;

use std::collections::BTreeMap;
use std::iter::Peekable;

use tinystr::{TinyStr4, TinyStr8};

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct TransformExtensionList {
    tlang: Option<LanguageIdentifier>,

    // Canonical: sort by key (BTreeMap is already) / remove value 'true'
    tfields: BTreeMap<TinyStr4, Vec<TinyStr8>>,
}

fn parse_tkey(key: &[u8]) -> Result<TinyStr4, ParserError> {
    if key.len() != 2 || !key[0].is_ascii_alphabetic() || !key[1].is_ascii_digit() {
        return Err(ParserError::InvalidSubtag);
    }
    let tkey = TinyStr4::from_bytes(key).map_err(|_| ParserError::InvalidSubtag)?;
    Ok(tkey.to_ascii_lowercase())
}

const TRUE_TVALUE: TinyStr8 = unsafe { TinyStr8::new_unchecked(1_702_195_828u64) }; // "true"

fn parse_tvalue(t: &[u8]) -> Result<Option<TinyStr8>, ParserError> {
    let s = TinyStr8::from_bytes(t).map_err(|_| ParserError::InvalidSubtag)?;
    if t.len() < 3 || t.len() > 8 || !s.is_ascii_alphanumeric() {
        return Err(ParserError::InvalidSubtag);
    }

    let s = s.to_ascii_lowercase();

    if s == TRUE_TVALUE {
        Ok(None)
    } else {
        Ok(Some(s))
    }
}

fn is_language_subtag(t: &[u8]) -> bool {
    let slen = t.len();
    (slen >= 2 && slen <= 8 || slen == 4) && !t.iter().any(|c: &u8| !c.is_ascii_alphabetic())
}

impl TransformExtensionList {
    pub fn is_empty(&self) -> bool {
        self.tlang.is_none() && self.tfields.is_empty()
    }

    pub fn set_tlang(&mut self, tlang: LanguageIdentifier) -> Result<(), LocaleError> {
        self.tlang = Some(tlang);
        Ok(())
    }

    pub fn set_tfield<S: AsRef<[u8]>>(&mut self, tkey: S, tvalue: &[S]) -> Result<(), LocaleError> {
        let tkey = parse_tkey(tkey.as_ref())?;
        let mut t = Vec::with_capacity(tvalue.len());
        for val in tvalue {
            if let Some(tval) = parse_tvalue(val.as_ref())? {
                t.push(tval);
            }
        }

        self.tfields.insert(tkey, t);
        Ok(())
    }

    pub(crate) fn try_from_iter<'a>(
        mut iter: &mut Peekable<impl Iterator<Item = &'a [u8]>>,
    ) -> Result<Self, ParserError> {
        let mut text = Self::default();

        let mut st_peek = iter.peek();

        let mut current_tkey = None;
        let mut current_tvalue = vec![];

        while let Some(subtag) = st_peek {
            let slen = subtag.len();
            if slen == 2 && subtag[0].is_ascii_alphabetic() && subtag[1].is_ascii_digit() {
                if let Some(current_tkey) = current_tkey {
                    text.tfields.insert(current_tkey, current_tvalue);
                    current_tvalue = vec![];
                }
                current_tkey = Some(parse_tkey(subtag)?);
                iter.next();
            } else if current_tkey.is_some() {
                if let Some(tval) = parse_tvalue(subtag)? {
                    current_tvalue.push(tval);
                }
                iter.next();
            } else if is_language_subtag(subtag) {
                text.tlang = Some(
                    LanguageIdentifier::try_from_iter(&mut iter, true)
                        .map_err(|_| ParserError::InvalidLanguage)?,
                );
            } else {
                break;
            }
            st_peek = iter.peek();
        }

        if let Some(current_keyword) = current_tkey {
            text.tfields.insert(current_keyword, current_tvalue);
        }

        Ok(text)
    }
}

impl std::fmt::Display for TransformExtensionList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        f.write_str("-t")?;

        if let Some(tlang) = &self.tlang {
            write!(f, "-{}", tlang)?;
        }

        for (k, t) in &self.tfields {
            write!(f, "-{}", k)?;
            for v in t {
                write!(f, "-{}", v)?;
            }
        }
        Ok(())
    }
}
