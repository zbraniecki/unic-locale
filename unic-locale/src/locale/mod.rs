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

pub fn serialize_locale(loc: &Locale) -> Result<String, LocaleError> {
    let langtag = serialize_langid(&loc.langid)?;
    let mut subtags = vec![langtag.as_str()];
    for (name, ext) in &loc.extensions {
        subtags.push(&extensions::convert_type_to_ext_type(&name).unwrap());

        for (key, value) in ext {
            subtags.push(&extensions::convert_key_to_ext_key(&key).unwrap());
            subtags.push(&value);
        }
    }

    Ok(subtags.join("-"))
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = serialize_locale(&self).unwrap();
        write!(f, "{}", result)
    }
}
