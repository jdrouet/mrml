use std::fmt;

use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::Text;

impl Serialize for Text {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

#[derive(Default)]
struct TextVisitor;

impl<'de> Visitor<'de> for TextVisitor {
    type Value = Text;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an map with properties type and children")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Text(value.to_string()))
    }
}

impl<'de> Deserialize<'de> for Text {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(TextVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::text::Text;

    #[test]
    fn serialize() {
        let elt = Text("Hello World".to_string());
        assert_eq!(serde_json::to_string(&elt).unwrap(), r#""Hello World""#);
    }

    #[test]
    fn deserialize() {
        let elt = Text("Hello World".to_string());
        let json = serde_json::to_string(&elt).unwrap();
        let res: Text = serde_json::from_str(&json).unwrap();
        assert_eq!(res.0, elt.0);
    }
}
