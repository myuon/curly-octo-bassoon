use proc_macro2::*;
use quote::quote;
use syn::{spanned::Spanned, Result};

use crate::{
    attr::get_label_options,
    util::{get_value, get_variant_names},
};

pub fn impl_from_str(ast: syn::DeriveInput) -> Result<TokenStream> {
    let span = ast.span();
    let variant_names = get_variant_names(ast.data, span)?;
    let options = get_label_options(ast.attrs)?;
    let name = &ast.ident;

    let branches = variant_names
        .into_iter()
        .map(|v| {
            let value = get_value(&v);

            quote!(
                #value => Ok(#name::#v),
            )
        })
        .collect::<Vec<_>>();
    let gen = quote! {
        impl FromStr for #name {
            type Err = &'static str;

            fn from_str(s: &str) -> Result<#name, Self::Err> {
                match s {
                    #(#branches)*
                    _ => Err(format!("Unknown variant: {}", s).as_str()),
                }
            }
        }
    };

    Ok(gen.into())
}

#[test]
fn test_impl_from_str() {
    let cases = vec![(
        r#"enum Test {
                A,
                B,
            }"#,
        quote! {
            impl FromStr for Test {
                type Err = &'static str;

                fn from_str(s: &str) -> Result<Test, Self::Err> {
                    match s {
                        "A" => Ok(Test::A),
                        "B" => Ok(Test::B),
                        _ => Err(format!("Unknown variant: {}", s).as_str()),
                    }
                }
            }
        },
    )];

    for tt in cases {
        pretty_assertions::assert_eq!(
            impl_from_str(syn::parse_str(tt.0).unwrap())
                .unwrap()
                .to_string(),
            tt.1.to_string()
        );
    }
}
