#[cfg(feature = "layout-inline")]
pub(super) mod layout_table;
#[cfg(feature = "likelysubtags-inline")]
pub(super) mod likelysubtags;

pub mod generate;

pub const CLDR_VERSION: &str = "36";
