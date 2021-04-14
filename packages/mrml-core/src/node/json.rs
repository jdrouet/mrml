use super::Node;
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

const FIELDS: [&str; 3] = ["type", "attributes", "children"];

impl<T: Serialize> Serialize for Node<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("type", &self.tag)?;
        map.serialize_entry("attributes", &self.attributes)?;
        map.serialize_entry("children", &self.children)?;
        map.end()
    }
}

struct NodeVisitor<T> {
    result: Node<T>,
}

impl<T> Default for NodeVisitor<T> {
    fn default() -> Self {
        Self {
            result: Node::<T>::from(String::default()),
        }
    }
}

impl<'de, T: Deserialize<'de>> Visitor<'de> for NodeVisitor<T> {
    type Value = Node<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an map with properties type, name, attributes and children")
    }

    fn visit_map<M>(mut self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        while let Some(key) = access.next_key::<String>()? {
            if key == "type" {
                self.result.tag = access.next_value()?;
            } else if key == "attributes" {
                self.result.attributes = access.next_value()?;
            } else if key == "children" {
                self.result.children = access.next_value()?;
            } else {
                return Err(M::Error::unknown_field(&key, &FIELDS));
            }
        }
        if self.result.tag.is_empty() {
            return Err(M::Error::missing_field("type"));
        }
        Ok(self.result)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Node<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(NodeVisitor::<T>::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::node::Node;
    use crate::text::Text;

    #[test]
    fn serialize() {
        let mut elt = Node::<Text>::from("span");
        elt.attributes.insert("margin-bottom".into(), "20px".into());
        elt.children.push(Text::from("Hello"));
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"span","attributes":{"margin-bottom":"20px"},"children":["Hello"]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"span","attributes":{"margin-bottom":"20px"},"children":["Hello","World","!"]}"#;
        let res: Node<Text> = serde_json::from_str(json).unwrap();
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.children.len(), 3);
    }
}
