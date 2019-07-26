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

    let lang = parsed.get_language();
    let lang = if lang.is_empty() {
        quote!(None)
    } else {
        quote!(Some(#lang))
    };
    let script = parsed.get_script();
    let script = if let Some(script) = script {
        quote!(Some(#script))
    } else {
        quote!(None)
    };
    let region = parsed.get_region();
    let region = if let Some(region) = region {
        quote!(Some(#region))
    } else {
        quote!(None)
    };
    let variants = parsed.get_variants();
    let variants = if variants.is_empty() {
        quote!(None)
    } else {
        quote!(Some(&[#(#variants,)*]))
    };

    TokenStream::from(quote! {
        ::unic_langid_impl::LanguageIdentifier::from_parts_unchecked(#lang, #script, #region, #variants)
    })
}
