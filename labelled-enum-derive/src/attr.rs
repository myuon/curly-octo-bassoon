use std::{collections::HashMap, panic};

use proc_macro2::{Ident, TokenStream};
use syn::{
    parse::{Parse, ParseStream},
    Result,
};

// attr example: label(rename_all = "{camelCase, snake_case, kebab-case}")

struct KeyValue {
    key: Ident,
    punct: syn::Token![=],
    value: String,
}

impl Parse for KeyValue {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(KeyValue {
            key: input.parse()?,
            punct: input.parse()?,
            value: input.parse::<syn::LitStr>()?.value(),
        })
    }
}

struct AttrInput {
    paren: syn::token::Paren,
    pairs: syn::punctuated::Punctuated<KeyValue, syn::Token![,]>,
}

impl Parse for AttrInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;

        Ok(AttrInput {
            paren: syn::parenthesized!(content in input),
            pairs: content.parse_terminated(KeyValue::parse)?,
        })
    }
}

pub enum Casing {
    CamelCase,
    SnakeCase,
    KebabCase,
}

pub struct LabelOptions {
    rename_all: Option<Casing>,
}

pub fn get_label_options(attrs: Vec<syn::Attribute>) -> Result<LabelOptions> {
    let mut result = LabelOptions { rename_all: None };
    for attr in attrs.into_iter() {
        let parsed = syn::parse2::<AttrInput>(attr.tokens)?;

        for pair in parsed.pairs {
            match pair.key.to_string().as_str() {
                "rename_all" => {
                    let casing = match pair.value.as_str() {
                        "camelCase" => Casing::CamelCase,
                        "snake_case" => Casing::SnakeCase,
                        "kebab-case" => Casing::KebabCase,
                        _ => panic!("Invalid value for rename_all"),
                    };
                    result.rename_all = Some(casing);
                }
                _ => panic!("Invalid key for label"),
            }
        }
    }

    Ok(result)
}
