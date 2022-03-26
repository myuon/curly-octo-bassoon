use proc_macro::TokenStream;
use syn::DeriveInput;

extern crate proc_macro;

mod attr;
mod from_str;
mod rename;
mod to_string;
mod util;

#[proc_macro_derive(ToString, attributes(label))]
pub fn to_string(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let result = to_string::impl_to_string(ast);
    match result {
        Ok(gen) => proc_macro::TokenStream::from(gen),
        Err(err) => panic!("{}", err),
    }
}

#[proc_macro_derive(FromStr, attributes(label))]
pub fn from_str(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let result = from_str::impl_from_str(ast);
    match result {
        Ok(gen) => proc_macro::TokenStream::from(gen),
        Err(err) => panic!("{}", err),
    }
}
