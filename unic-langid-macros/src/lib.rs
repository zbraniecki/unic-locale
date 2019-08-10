use proc_macro_hack::proc_macro_hack;
pub use tinystr::{TinyStr4, TinyStr8};
pub use unic_langid_impl::LanguageIdentifier;

/// Add one to an expression.
///
/// (Documentation goes here on the re-export, not in the other crate.)
#[proc_macro_hack]
pub use unic_langid_macros_impl::langid;
