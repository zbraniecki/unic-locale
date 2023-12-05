use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use unic_locale_impl::Locale;

const CRATE_NAME: &[&str] = &[
    "unic-locale",
    "unic-locale-impl",
];

pub(crate) fn get_crate_name() -> Ident {
    let name = find_crate::find_crate(|s| CRATE_NAME.contains(&s))
        .expect("Failed to find the crate in Cargo.toml")
        .name;
    Ident::new(&name, Span::call_site())
}

pub(crate) fn extract_string(s: TokenStream) -> String {
    let s = s.to_string();
    let result = if s.len() > 2 {
        &s[1..s.len() - 1]
    } else {
        s.as_str()
    };
    result.to_string()
}

pub fn locale_impl(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let krate = get_crate_name();
    let input = extract_string(input);
    let parsed: Locale = input.parse().expect("Malformed Locale Identifier");

    let (lang, script, region, variants, extensions) = parsed.into_parts();

    let lang: Option<u64> = lang.into();
    let lang = if let Some(lang) = lang {
        quote!(unsafe { ::#krate::subtags::Language::from_raw_unchecked(#lang) })
    } else {
        quote!($crate::subtags::Language::default())
    };
    let script = if let Some(script) = script {
        let script: u32 = script.into();
        quote!(Some(unsafe { ::#krate::subtags::Script::from_raw_unchecked(#script) }))
    } else {
        quote!(None)
    };
    let region = if let Some(region) = region {
        let region: u32 = region.into();
        quote!(Some(unsafe { ::#krate::subtags::Region::from_raw_unchecked(#region) }))
    } else {
        quote!(None)
    };
    let variants = if !variants.is_empty() {
        let v: Vec<_> = variants
            .iter()
            .map(|v| {
                let variant: u64 = v.into();
                quote!(unsafe { ::#krate::subtags::Variant::from_raw_unchecked(#variant) })
            })
            .collect();
        quote!(Some(Box::new([#(#v,)*])))
    } else {
        quote!(None)
    };

    quote! {
        unsafe { ::#krate::Locale::from_raw_parts_unchecked(
            #lang,
            #script,
            #region,
            #variants,
            #extensions.parse().expect("must parse")
        ) }
    }
}
