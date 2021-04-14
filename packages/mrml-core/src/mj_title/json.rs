use super::{MJTitle, NAME};
use crate::json_children_serializer;
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer};
use std::fmt;

const FIELDS: [&str; 2] = ["type", "children"];

json_children_serializer!(MJTitle, NAME);

#[derive(Default)]
struct MJTitleVisitor;

impl<'de> Visitor<'de> for MJTitleVisitor {
    type Value = MJTitle;

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
        Ok(MJTitle::from(result))
    }
}

impl<'de> Deserialize<'de> for MJTitle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MJTitleVisitor::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_title::MJTitle;

    #[test]
    fn serialize() {
        let elt = MJTitle::from("Hello World");
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-title","children":"Hello World"}"#
        );
    }

    #[test]
    fn deserialize() {
        let elt = MJTitle::from("Hello World");
        let json = serde_json::to_string(&elt).unwrap();
        let res: MJTitle = serde_json::from_str(&json).unwrap();
        assert_eq!(res.children, elt.children);
    }
}
