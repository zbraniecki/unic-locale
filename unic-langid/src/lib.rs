//! `unic-langid` is a core API for parsing, manipulating, and serializing Unicode Language
//! Identifiers.
//!
//! The crate provides algorithms for parsing a string into a well-formed language identifier
//! as defined by LDML UTS #35.
//!
//! # Examples
//!
//! ```
//! use unic_langid::LanguageIdentifier;
//!
//! let mut li: LanguageIdentifier = "en-US".parse()
//!     .expect("Failed to parse.");
//!
//! assert_eq!(li.get_language(), "en");
//! assert_eq!(li.get_script(), None);
//! assert_eq!(li.get_region(), Some("US"));
//! assert_eq!(li.get_variants().len(), 0);
//!
//! li.set_region(Some("GB"))
//!     .expect("Region parsing failed.");
//!
//! assert_eq!(li.to_string(), "en-GB");
//! ```
//!
//! For more details, see `LanguageIdentifier`.

pub use unic_langid_impl::*;

#[cfg(feature = "unic-langid-macros")]
pub use unic_langid_macros::langid;

#[cfg(feature = "unic-langid-macros")]
#[macro_export]
macro_rules! langids {
    ( $($langid:expr),* ) => {
        {
            let mut v = vec![];
            $(
                v.push(langid!($langid));
            )*
            v
        }
    };
}
