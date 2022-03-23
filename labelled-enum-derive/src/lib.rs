use proc_macro::TokenStream;
use syn::DeriveInput;

extern crate proc_macro;

mod to_string;

#[proc_macro_derive(ToString)]
pub fn to_string(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let result = to_string::impl_to_string(&ast);
    match result {
        Ok(gen) => proc_macro::TokenStream::from(gen),
        Err(err) => panic!("{}", err),
    }
}
