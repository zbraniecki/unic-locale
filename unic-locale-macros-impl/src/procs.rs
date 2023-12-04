use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use unic_locale_impl::Locale;

const CRATE_NAME: &str = "unic-locale-impl";

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

pub fn locale_impl(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let krate = Ident::new(&get_crate_name(), Span::call_site());
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

    TokenStream::from(quote! {
        unsafe { ::#krate::Locale::from_raw_parts_unchecked(
            #lang,
            #script,
            #region,
            #variants,
            #extensions.parse().expect("must parse")
        ) }
    })
}
