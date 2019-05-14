#[derive(Debug, PartialEq)]
pub struct Locale {
    pub langid: LanguageIdentifier,
    pub extensions: BTreeMap<String, BTreeMap<String, String>>,
}

impl Locale {
    pub fn from_ident_with_options(
        ident: &str,
        options: BTreeMap<&str, &str>,
    ) -> Result<Self, LocaleError> {
        let mut loc = parser::parse_locale(ident)?;
        for (key, value) in options {
            if key == "language" {
                loc.langid.set_language(Some(value))?;
            } else if key == "script" {
                loc.langid.set_script(Some(value))?;
            } else if key == "region" {
                loc.langid.set_region(Some(value))?;
            } else {
                let extension = "unicode";
                loc.set_extension(extension, key, value)?;
            }
        }
        Ok(loc)
    }

    pub fn set_extension(
        &mut self,
        extension: &str,
        key: &str,
        value: &str,
    ) -> Result<(), LocaleError> {
        let ext = self
            .extensions
            .entry(extension.to_string())
            .or_insert(BTreeMap::new());
        ext.insert(key.to_string(), value.to_string());
        Ok(())
    }
}
