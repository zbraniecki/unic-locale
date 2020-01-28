extern crate proc_macro;

use proc_macro::TokenStream;

use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::{parse_macro_input, LitStr};

use unic_langid_impl::LanguageIdentifier;

#[proc_macro_hack]
pub fn langid(input: TokenStream) -> TokenStream {
    let id = parse_macro_input!(input as LitStr);
    let parsed: LanguageIdentifier = id.value().parse().expect("Malformed Language Identifier");

    let (lang, script, region, variants) = parsed.into_raw_parts();
    let lang = if let Some(lang) = lang {
        quote!(Some(unsafe { $crate::TinyStr8::new_unchecked(#lang) }))
    } else {
        quote!(None)
    };
    let script = if let Some(script) = script {
        quote!(Some(unsafe { $crate::TinyStr4::new_unchecked(#script) }))
    } else {
        quote!(None)
    };
    let region = if let Some(region) = region {
        quote!(Some(unsafe { $crate::TinyStr4::new_unchecked(#region) }))
    } else {
        quote!(None)
    };
    let variants = if let Some(variants) = variants {
        let v: Vec<_> = variants
            .iter()
            .map(|v| quote!(unsafe { $crate::TinyStr8::new_unchecked(#v) }))
            .collect();
        quote!(Some(Box::new([#(#v,)*])))
    } else {
        quote!(None)
    };

    TokenStream::from(quote! {
    $crate::LanguageIdentifier::from_raw_parts_unchecked(#lang, #script, #region, #variants) })
}
