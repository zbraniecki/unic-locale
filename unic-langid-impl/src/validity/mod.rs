use crate::LanguageIdentifier;

pub trait IsValid {
  fn is_valid(&self) -> Option<bool>;

  fn validate(&mut self) -> Result<(), ()>;
}

impl IsValid for LanguageIdentifier {
  fn is_valid(&self) -> Option<bool> {
    self.is_valid
  }

  fn validate(&mut self) -> Result<(), ()> {
    // Here's we'll pull:
    //  * ISO 639-1 or ISO 639-2 for language codes validation
    //  * ISO-15924 for scripts
    //  * ISO-3166 for regions
    self.is_valid = Some(true);
    Ok(())
  }
}