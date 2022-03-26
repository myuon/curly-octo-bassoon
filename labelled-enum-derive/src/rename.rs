use convert_case::{Case, Casing as ConvertCaseCasing};

pub enum Casing {
    CamelCase,
    SnakeCase,
    KebabCase,
}

pub fn rename(casing: &Option<Casing>, input: String) -> String {
    if let Some(c) = casing {
        input.to_case(match c {
            Casing::CamelCase => Case::Camel,
            Casing::SnakeCase => Case::Snake,
            Casing::KebabCase => Case::Kebab,
        })
    } else {
        input
    }
}
