use proc_macro::TokenStream;
use syn::DeriveInput;

extern crate proc_macro;

mod from_str;
mod to_string;
mod util;

#[proc_macro_derive(ToString)]
pub fn to_string(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let result = to_string::impl_to_string(&ast);
    match result {
        Ok(gen) => proc_macro::TokenStream::from(gen),
        Err(err) => panic!("{}", err),
    }
}

#[proc_macro_derive(FromStr)]
pub fn from_str(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let result = from_str::impl_from_str(&ast);
    match result {
        Ok(gen) => proc_macro::TokenStream::from(gen),
        Err(err) => panic!("{}", err),
    }
}
