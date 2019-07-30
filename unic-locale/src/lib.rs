pub use unic_locale_impl::*;

#[cfg(feature = "unic-locale-macros")]
pub use unic_locale_macros::locale;

#[cfg(feature = "unic-locale-macros")]
#[macro_export]
macro_rules! locales {
    ( $($loc:expr),* ) => {
        {
            let mut v = vec![];
            $(
                v.push(locale!($loc));
            )*
            v
        }
    };
}
