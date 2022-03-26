use proc_macro2::Ident;
use syn::Data;

pub fn get_variant_names(input: Data) -> Result<Vec<Ident>, &'static str> {
    let data = match input {
        syn::Data::Enum(d) => d,
        _ => return Err("#[derive(ToString)] is only defined for enums"),
    };

    let mut result = vec![];
    for variant in data.variants.iter() {
        if !variant.fields.is_empty() {
            return Err("#[derive(ToString)] does not support variant with fields");
        }
        if variant.discriminant.is_some() {
            return Err("#[derive(ToString)] does not support variant discriminants");
        }

        result.push(variant.ident.clone());
    }

    Ok(result)
}

pub fn get_value(variant: &Ident) -> String {
    variant.to_string()
}
