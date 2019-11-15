#[cfg(feature = "layout-inline")]
pub(super) mod layout_table;
#[cfg(feature = "likelysubtags-inline")]
pub(super) mod likelysubtags;

#[cfg(feature = "binary")]
pub mod generate;
mod version;

pub use version::CLDR_VERSION;
