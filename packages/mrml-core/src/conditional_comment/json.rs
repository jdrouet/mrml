use std::fmt;

use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::ConditionalComment;

impl Serialize for ConditionalComment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

#[derive(Default)]
struct ConditionalCommentVisitor;

impl Visitor<'_> for ConditionalCommentVisitor {
    type Value = ConditionalComment;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an map with properties type and children")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(ConditionalComment(value.to_string()))
    }
}

impl<'de> Deserialize<'de> for ConditionalComment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ConditionalCommentVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::conditional_comment::ConditionalComment;

    #[test]
    fn serialize() {
        let elt = ConditionalComment("<![endif]-->".to_string());
        assert_eq!(serde_json::to_string(&elt).unwrap(), r#""<![endif]-->""#);
    }

    #[test]
    fn deserialize() {
        let elt = ConditionalComment("<!--[if IE]>".to_string());
        let json = serde_json::to_string(&elt).unwrap();
        let res: ConditionalComment = serde_json::from_str(&json).unwrap();
        assert_eq!(res.0, elt.0);
    }
}
