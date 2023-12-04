extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod procs;

#[proc_macro]
pub fn locale(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    procs::locale_impl(input).into()
}
