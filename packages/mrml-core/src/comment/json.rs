use super::Comment;
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

const NAME: &str = "comment";
const FIELDS: [&str; 2] = ["type", "children"];

impl Serialize for Comment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("type", NAME)?;
        map.serialize_entry("children", &self.0)?;
        map.end()
    }
}

#[derive(Default)]
struct CommentVisitor;

impl<'de> Visitor<'de> for CommentVisitor {
    type Value = Comment;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an map with properties type and children")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut result = String::default();
        while let Some((key, value)) = access.next_entry::<String, String>()? {
            if key == "type" {
                if value != NAME {
                    return Err(M::Error::custom(format!("expected type to equal {}", NAME)));
                }
            } else if key == "children" {
                result = value;
            } else {
                return Err(M::Error::unknown_field(&key, &FIELDS));
            }
        }
        Ok(Comment(result))
    }
}

impl<'de> Deserialize<'de> for Comment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(CommentVisitor::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::comment::Comment;

    #[test]
    fn serialize() {
        let elt = Comment("Hello World".to_string());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"comment","children":"Hello World"}"#
        );
    }

    #[test]
    fn deserialize() {
        let elt = Comment("Hello World".to_string());
        let json = serde_json::to_string(&elt).unwrap();
        let res: Comment = serde_json::from_str(&json).unwrap();
        assert_eq!(res.0, elt.0);
    }
}
