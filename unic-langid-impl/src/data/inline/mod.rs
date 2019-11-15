#[cfg(feature = "layout-inline")]
pub(super) mod layout_table;
#[cfg(feature = "likelysubtags-inline")]
pub(super) mod likelysubtags;

mod version;

pub use version::CLDR_VERSION;
