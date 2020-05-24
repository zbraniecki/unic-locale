//! `unic-locale` is an API for parsing, manipulating, and serializing Unicode Locale
//! Identifiers.
//!
//! The crate provides algorithms for parsing a string into a well-formed locale identifier
//! as defined by [`UTS #35: Unicode LDML 3.1 Unicode Locale Identifier`].
//!
//! # Locale vs. LanguageIdentifier
//!
//! `LanguageIdentifier` is a subset of a `Locale` that only provides the basic
//! subtags such as `language`, `script`, `region` and `variants`.
//!
//! `Locale` extends that with a set of extensions such as `transform`, `unicode` and `private`.
//!
//!
//! # Examples
//!
//! ```
//! use unic_locale::Locale;
//!
//! let mut loc: Locale = "en-Latn-US-u-hc-h12-t-h0-hybrid".parse()
//!     .expect("Failed to parse.");
//!
//! assert_eq!(loc.id.language.as_str(), "en");
//! assert_eq!(loc.id.script, Some("Latn".parse().unwrap()));
//! assert_eq!(loc.id.region, Some("US".parse().unwrap()));
//! assert_eq!(loc.id.variants().len(), 0);
//! assert_eq!(loc.extensions.unicode.keyword("hc")
//!     .expect("Getting keyword failed.")
//!     .collect::<Vec<_>>(), &["h12"]);
//! assert_eq!(loc.extensions.transform.tfield("h0")
//!     .expect("Getting tfield failed.")
//!     .collect::<Vec<_>>(), &["hybrid"]);
//!
//! loc.id.script = None;
//! loc.id.region = Some("GB".parse().expect("Region parsing failed."));
//!
//! assert_eq!(loc.to_string(), "en-GB-t-h0-hybrid-u-hc-h12");
//! ```
//!
//! For more details, see [`Locale`].
//!
//! # Optional features
//!
//! ## `locale!` and `locale!` macros
//!
//! If `feature = "macros"` is selected, the crate provides a procedural macro
//! which allows to construct build-time well-formed locale identifiers with zero-cost at runtime.
//!
//! ``` ignore
//! use unic_locale::{locale, locales};
//!
//! let es_ar = locale!("es-AR");
//! let en_us = locale!("en-US");
//!
//! assert_eq!(es_ar, "es-AR");
//! assert_eq!(en_us, "en-US");
//!
//! let locales = locales!("es-AR", "en-US", "de");
//!
//! assert_eq!(locales.get(0), "es-AR");
//! assert_eq!(locales.get(1), "en-US");
//! assert_eq!(locales.get(2), "de");
//! ```
//!
//! The macros produce instances of `Locale` the same way as parsing from `&str` does,
//! but since the parsing is performed at build time, it doesn't need a `Result`.
//!
//! Unlike `langid!` `locale!` can't be used for const variables.
//!
//! The macros are optional to reduce the dependency chain and compilation time of `unic-locale`.
//!
//! ## Likely Subtags
//!
//! If `feature = "likelysubtags"` is selected, the `Locale` gains two more methods:
//!
//!  * add_likely_subtags
//!  * remove_likely_subtags
//!
//! Both of them operate in place updating the existing `Locale` by either extending
//! subtags to most likely values, or removing the subtags that are not needed.
//!
//! Both methods return a `bool` that indicates if the identifier has been modified.
//!
//! ``` ignore
//! use unic_locale::Locale;
//!
//! let mut loc: Locale = "fr-FR".parse()
//!     .expect("Parsing failed.");
//!
//! assert_eq!(loc.add_likely_subtags(), true);
//! assert_eq!(loc, "fr-Latn-FR");
//!
//! assert_eq!(loc.remove_likely_subtags(), true);
//! assert_eq!(loc, "fr");
//! ```
//!
//! The feature is optional because it increases the binary size of the library by including
//! a data table for CLDR likelySubtags.
//!
//! [`UTS #35: Unicode LDML 3.1 Unicode Locale Identifier`]: https://unicode.org/reports/tr35/tr35.html#Unicode_locale_identifier
//! [`Locale`]: ./struct.Locale.html
pub use unic_locale_impl::*;

#[cfg(feature = "unic-locale-macros")]
pub use unic_locale_macros::locale;

#[cfg(feature = "unic-locale-macros")]
#[macro_export]
macro_rules! locales {
    ( $($locale:expr),* ) => {
        vec![$(
            $crate::locale!($locale),
        )*]
    };
    ( $($locale:expr,)* ) => {
        $crate::locales![$($locale),*]
    };
}
