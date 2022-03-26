use proc_macro2::{Ident, Span};
use syn::{spanned::Spanned, Data, Error, Result};

pub fn get_variant_names(input: Data, span: Span) -> Result<Vec<Ident>> {
    let data = match input {
        syn::Data::Enum(d) => d,
        _ => {
            return Err(Error::new(
                span,
                "#[derive(ToString)] is only defined for enums",
            ))
        }
    };

    let mut result = vec![];
    for variant in data.variants.iter() {
        if !variant.fields.is_empty() {
            return Err(Error::new(
                variant.fields.span(),
                "#[derive(ToString)] does not support variant with fields",
            ));
        }
        if variant.discriminant.is_some() {
            return Err(Error::new(
                variant.discriminant.as_ref().unwrap().0.span(),
                "#[derive(ToString)] does not support variant discriminants",
            ));
        }

        result.push(variant.ident.clone());
    }

    Ok(result)
}

pub fn get_value(variant: &Ident) -> String {
    variant.to_string()
}
