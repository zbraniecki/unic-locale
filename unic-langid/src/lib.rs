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
