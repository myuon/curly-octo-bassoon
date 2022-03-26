# labelled-enum [![labelled-enum at crates.io](https://img.shields.io/crates/v/labelled-enum.svg)](https://crates.io/crates/labelled-enum) [![labelled-enum at docs.rs](https://docs.rs/labelled-enum/badge.svg)](https://docs.rs/labelled-enum)

Converting an enum to/from String.

## Getting Started

Derives ToString, FromStr impl:

```rust
#[derive(ToString, FromStr)]
enum Test {
    Foo,
    Bar,
}

assert_eq!(Test::Foo.to_string(), "Foo");
assert_eq!(Test::from_str("Foo").unwrap(), Test::Foo);
```

## Casing

You can specify snake_case using attribute:

```rust
#[derive(ToString, FromStr)]
#[label(rename_all = "snake_case")]
enum Test {
    SnakeCase,
}
```

## Working with [serde](https://serde.rs)

labelled-enum provides `serde_plugin` feature to work with serde Serializer/Deserializer:

```rust
// install labelled-enum with --features serde_plugin

#[derive(Serialize, Deserialize)]
struct Wrapper {
    #[serde(with = "labelled_enum::serde_plugin")]
    test_snake_case: TestSnakeCase,
}
```
