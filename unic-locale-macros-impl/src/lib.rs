extern crate proc_macro;

use proc_macro::TokenStream;

use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::{parse_macro_input, LitStr};

use unic_locale_impl::Locale;

#[proc_macro_hack]
pub fn locale(input: TokenStream) -> TokenStream {
    let id = parse_macro_input!(input as LitStr);
    let parsed: Locale = id.value().parse().expect("Malformed Locale Identifier");

    let (lang, script, region, variants, extensions) = parsed.into_parts();

    let lang: Option<u64> = lang.into();
    let lang = if let Some(lang) = lang {
        quote!(unsafe { $crate::subtags::Language::from_raw_unchecked(#lang) })
    } else {
        quote!($crate::subtags::Language::default())
    };
    let script = if let Some(script) = script {
        let script: u32 = script.into();
        quote!(Some(unsafe { $crate::subtags::Script::from_raw_unchecked(#script) }))
    } else {
        quote!(None)
    };
    let region = if let Some(region) = region {
        let region: u32 = region.into();
        quote!(Some(unsafe { $crate::subtags::Region::from_raw_unchecked(#region) }))
    } else {
        quote!(None)
    };
    let variants = if !variants.is_empty() {
        let v: Vec<_> = variants
            .iter()
            .map(|v| {
                let variant: u64 = v.into();
                quote!(unsafe { $crate::subtags::Variant::from_raw_unchecked(#variant) })
            })
            .collect();
        quote!(Some(Box::new([#(#v,)*])))
    } else {
        quote!(None)
    };

    TokenStream::from(quote! {
        unsafe { $crate::Locale::from_raw_parts_unchecked(
            #lang,
            #script,
            #region,
            #variants,
            #extensions.parse().expect("must parse")
        ) }
    })
}
