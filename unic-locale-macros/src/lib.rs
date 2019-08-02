use proc_macro_hack::proc_macro_hack;
pub use unic_locale_impl::Locale;

/// Add one to an expression.
///
/// (Documentation goes here on the re-export, not in the other crate.)
#[proc_macro_hack]
pub use unic_locale_macros_impl::locale;
