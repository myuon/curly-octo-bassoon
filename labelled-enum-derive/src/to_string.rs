use proc_macro2::*;
use quote::quote;
use syn::DataEnum;

fn get_variant_names(data: &DataEnum) -> Vec<Ident> {
    let mut result = vec![];
    for variant in data.variants.iter() {
        if !variant.fields.is_empty() {
            panic!("#[derive(ToString)] does not support variant with fields");
        }
        if variant.discriminant.is_some() {
            panic!("#[derive(ToString)] does not support variant discriminants");
        }

        result.push(variant.ident.clone());
    }

    result
}

fn get_value(variant: &Ident) -> String {
    variant.to_string()
}

pub fn impl_to_string(ast: &syn::DeriveInput) -> Result<TokenStream, &str> {
    let data = match &ast.data {
        syn::Data::Enum(d) => d,
        _ => return Err("#[derive(ToString)] is only defined for enums"),
    };
    let variant_names = get_variant_names(data);

    let name = &ast.ident;

    let branches = variant_names
        .into_iter()
        .map(|v| {
            let value = get_value(&v);

            quote!(
                #name::#v => #value.to_string(),
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

    Ok(gen.into())
}

#[test]
fn test_impl_to_string() {
    let cases = vec![(
        r#"enum Test {
                A,
                B,
            }"#,
        quote! {
            impl ToString for Test {
                fn to_string(&self) -> String {
                    match self {
                        Test::A => "A".to_string(),
                        Test::B => "B".to_string(),
                    }
                }
            }
        },
    )];

    for tt in cases {
        pretty_assertions::assert_eq!(
            impl_to_string(&syn::parse_str(tt.0).unwrap())
                .unwrap()
                .to_string(),
            tt.1.to_string()
        );
    }
}
