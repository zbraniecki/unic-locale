use crate::parser::ParserError;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct PrivateExtensionList(Vec<String>);

impl PrivateExtensionList {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn try_from_iter<'a>(
        iter: &mut impl Iterator<Item = &'a str>,
    ) -> Result<Self, ParserError> {
        let mut text = Self::default();

        for subtag in iter {
            text.0.push(subtag.to_ascii_lowercase());
        }

        Ok(text)
    }

    pub fn insert(&mut self, value: String) -> Result<(), ParserError> {
        self.0.push(value);
        Ok(())
    }
}

impl std::fmt::Display for PrivateExtensionList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        f.write_str("-x")?;

        for subtag in &self.0 {
            write!(f, "-{}", subtag)?;
        }
        Ok(())
    }
}
