#[cfg(any(feature = "layout-inline", feature = "likelysubtags-inline"))]
mod inline;
// mod cldr;

#[cfg(any(feature = "layout-inline", feature = "layout-cldr"))]
pub(crate) use inline::layout_table::CHARACTER_DIRECTION_RTL;

#[cfg(any(feature = "likelysubtags-inline", feature = "likelysubtags-cldr"))]
pub(crate) use inline::likelysubtags::{add_likely_subtags, remove_likely_subtags};

#[cfg(any(feature = "layout-inline", feature = "likelysubtags-inline"))]
pub use inline::CLDR_VERSION;
