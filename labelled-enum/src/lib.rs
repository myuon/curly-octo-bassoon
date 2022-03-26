pub use labelled_enum_derive::*;

// Working with serde
pub mod serde_plugin {
    use serde::*;
    use std::str::FromStr;

    pub fn serialize<S>(data: &impl ToString, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&data.to_string())
    }

    pub fn deserialize<'de, E, D>(deserializer: D) -> Result<E, D::Error>
    where
        D: Deserializer<'de>,
        E: FromStr<Err = String>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}
