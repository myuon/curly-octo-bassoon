use labelled_enum::*;
use std::str::FromStr;

#[derive(ToString, FromStr, PartialEq, Debug)]
enum Test {
    Foo,
    Bar,
}

#[test]
fn test_basic() {
    assert_eq!(Test::Foo.to_string(), "Foo");
    assert_eq!(Test::from_str("Foo").unwrap(), Test::Foo);
}

#[derive(ToString, FromStr, Debug, PartialEq)]
#[label(rename_all = "snake_case")]
enum TestSnakeCase {
    SnakeCase,
}

#[test]
fn test_snake_case() {
    assert_eq!(TestSnakeCase::SnakeCase.to_string(), "snake_case");
    assert_eq!(
        TestSnakeCase::from_str("snake_case").unwrap(),
        TestSnakeCase::SnakeCase
    );
}
