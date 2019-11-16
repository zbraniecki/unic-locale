#[cfg(any(feature = "layout-inline", feature = "likelysubtags-inline",))]
mod inline;

#[cfg(any(feature = "layout-cldr", feature = "likelysubtags-cldr"))]
mod cldr;

#[cfg(any(
    feature = "binary",
    feature = "layout-cldr",
    feature = "likelysubtags-cldr"
))]
pub mod generate;

#[cfg(feature = "layout-cldr")]
pub(crate) use cldr::layout_table::is_rtl;
#[cfg(feature = "layout-inline")]
pub(crate) use inline::layout_table::is_rtl;

#[cfg(feature = "likelysubtags-cldr")]
pub(crate) use cldr::likelysubtags::{add_likely_subtags, remove_likely_subtags};
#[cfg(feature = "likelysubtags-inline")]
pub(crate) use inline::likelysubtags::{add_likely_subtags, remove_likely_subtags};

#[cfg(any(feature = "layout-inline", feature = "likelysubtags-inline"))]
pub use inline::CLDR_VERSION;
