use proc_macro2::*;
use quote::quote;
use syn::DataEnum;

pub fn get_variant_names(data: &DataEnum) -> Vec<String> {
    let mut result = vec![];
    for variant in data.variants.iter() {
        if !variant.fields.is_empty() {
            panic!("#[derive(ToString)] does not support variant with fields");
        }
        if variant.discriminant.is_some() {
            panic!("#[derive(ToString)] does not support variant discriminants");
        }

        result.push(variant.ident.to_string());
    }

    result
}

pub fn impl_to_string(ast: &syn::DeriveInput) -> TokenStream {
    let data = match &ast.data {
        syn::Data::Enum(d) => d,
        _ => panic!("#[derive(ToString)] is only defined for enums"),
    };
    let variant_names = get_variant_names(data);

    let name = &ast.ident;

    let branches = variant_names
        .into_iter()
        .map(|v| {
            quote!(
                #name::#v => #v.to_string(),
            )
        })
        .collect::<Vec<_>>();
    let gen = quote! {
        impl ToString for #name {
            fn to_string(&self) -> String {
                match self {
                    #(#branches)*
                }
            }
        }
    };

    gen.into()
}
