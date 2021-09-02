use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Op {
    #[serde(flatten)]
    pub content: OpContent,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub attributes: BTreeMap<String, serde_json::Value>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OpContent {
    Insert(serde_json::Value),
}

#[derive(Deserialize, Serialize)]
pub struct Delta {
    pub ops: Vec<Op>,
}

impl Delta {
    pub fn plain_text(&self) -> String {
        self.ops
            .iter()
            .filter_map(|op| match &op.content {
                OpContent::Insert(value) => match value {
                    serde_json::Value::String(str) => Some(str.as_str()),
                    _ => None,
                },
                // For the future: there can be other kinds of operations.
                _ => None,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Delta, Op, OpContent};

    const D1: &str = r#"{
      "ops": [ { "insert": "Hello\n\nLet's write some code!\n" } ]
    }"#;

    const D2: &str = r#"{
      "ops": [
        {
          "attributes": {
            "bold": true
          },
          "insert": "Hello"
        },
        {
          "insert": "\n\nLet's write some "
        },
        {
          "attributes": {
            "italic": true
          },
          "insert": "code"
        },
        {
          "insert": "!\n"
        }
      ]
    }"#;

    #[test]
    fn deserialize() {
        let d1_delta: Delta = serde_json::from_str(D1).unwrap();
        assert_eq!(d1_delta.ops.len(), 1);
        assert_eq!(
            d1_delta.ops[0],
            Op {
                content: OpContent::Insert(serde_json::Value::String(String::from(
                    "Hello\n\nLet's write some code!\n"
                ))),
                attributes: std::collections::BTreeMap::new(),
            },
        );

        let d2_delta: Delta = serde_json::from_str(D2).unwrap();
        let mut d2_attributes = std::collections::BTreeMap::new();
        d2_attributes.insert(String::from("bold"), serde_json::Value::Bool(true));
        assert_eq!(d2_delta.ops.len(), 4);
        assert_eq!(
            d2_delta.ops[0],
            Op {
                content: OpContent::Insert(serde_json::Value::String(String::from("Hello"))),
                attributes: d2_attributes,
            },
        );
    }

    #[test]
    fn serialize() {
        let d1_delta: Delta = serde_json::from_str(D1).unwrap();
        assert_eq!(
            serde_json::to_string(&d1_delta).unwrap(),
            r#"{"ops":[{"insert":"Hello\n\nLet's write some code!\n"}]}"#
        );
    }

    #[test]
    fn plain_text() {
        let d1_delta: Delta = serde_json::from_str(D1).unwrap();
        assert_eq!(d1_delta.plain_text(), "Hello\n\nLet's write some code!\n");

        // Same text, no formatting.
        let d2_delta: Delta = serde_json::from_str(D2).unwrap();
        assert_eq!(d2_delta.plain_text(), "Hello\n\nLet's write some code!\n");
    }
}
