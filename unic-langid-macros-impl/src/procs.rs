use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use unic_langid_impl::{subtags, LanguageIdentifier};

const CRATE_NAME: &str = "unic-langid-impl";

pub(crate) fn get_crate_name() -> String {
    let found_crate = proc_macro_crate::crate_name(CRATE_NAME)
        .unwrap_or_else(|_| panic!("{}", "{CRATE_NAME} is present in `Cargo.toml`"));

    match found_crate {
        proc_macro_crate::FoundCrate::Itself => CRATE_NAME.to_string(),
        proc_macro_crate::FoundCrate::Name(name) => name,
    }
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

pub fn lang_impl(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let krate = Ident::new(&get_crate_name(), Span::call_site());
    let input = extract_string(input);
    let parsed: subtags::Language = input.parse().expect("Malformed Language Subtag");

    let lang: Option<u64> = parsed.into();
    let lang = if let Some(lang) = lang {
        quote!(unsafe { ::#krate::subtags::Language::from_raw_unchecked(#lang) })
    } else {
        quote!(None)
    };

    TokenStream::from(quote! {
        #lang
    })
}

pub fn script_impl(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let krate = Ident::new(&get_crate_name(), Span::call_site());
    let input = extract_string(input);
    let parsed: subtags::Script = input.parse().expect("Malformed Script Subtag");

    let script: u32 = parsed.into();

    quote! {
        unsafe { ::#krate::subtags::Script::from_raw_unchecked(#script) }
    }
}

pub fn region_impl(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let krate = Ident::new(&get_crate_name(), Span::call_site());
    let input = extract_string(input);
    let parsed: subtags::Region = input.parse().expect("Malformed Region Subtag");

    let region: u32 = parsed.into();

    quote! {
        unsafe { ::#krate::subtags::Region::from_raw_unchecked(#region) }
    }
}

pub fn variant_impl(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let krate = Ident::new(&get_crate_name(), Span::call_site());
    let input = extract_string(input);
    let parsed: subtags::Variant = input.parse().expect("Malformed Variant Subtag");

    let variant: u64 = parsed.into();

    quote! {
        unsafe { ::#krate::subtags::Variant::from_raw_unchecked(#variant) }
    }
}

pub fn langid_impl(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let krate = Ident::new(&get_crate_name(), Span::call_site());
    let input = extract_string(input);
    let parsed: LanguageIdentifier = input.parse().expect("Malformed Language Identifier");

    let (lang, script, region, variants) = parsed.into_parts();

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
        unsafe { ::#krate::LanguageIdentifier::from_raw_parts_unchecked(#lang, #script, #region, #variants) }
    }
}
