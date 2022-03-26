use labelled_enum::{FromStr, ToString};
use serde::*;

#[derive(ToString, FromStr, Debug, PartialEq)]
#[label(rename_all = "snake_case")]
enum TestSnakeCase {
    SnakeCase,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Wrapper {
    #[serde(with = "labelled_enum::serde_plugin")]
    test_snake_case: TestSnakeCase,
}

#[test]
fn test_serde() {
    assert_eq!(
        serde_json::to_string(&Wrapper {
            test_snake_case: TestSnakeCase::SnakeCase,
        })
        .unwrap(),
        "{\"testSnakeCase\":\"snake_case\"}"
    );
    assert_eq!(
        serde_json::from_str::<Wrapper>(r#"{"testSnakeCase": "snake_case"}"#).unwrap(),
        Wrapper {
            test_snake_case: TestSnakeCase::SnakeCase,
        }
    );
}
