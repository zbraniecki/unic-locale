extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod procs;

#[proc_macro]
pub fn lang(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    procs::lang_impl(input).into()
}

#[proc_macro]
pub fn script(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    procs::script_impl(input).into()
}

#[proc_macro]
pub fn region(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    procs::region_impl(input).into()
}

#[proc_macro]
pub fn variant_fn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    procs::variant_impl(input).into()
}

#[proc_macro]
pub fn langid(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    procs::langid_impl(input).into()
}
