use super::{MJAccordion, NAME};
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

const FIELDS: [&str; 3] = ["type", "attributes", "children"];

impl Serialize for MJAccordion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("type", NAME)?;
        map.serialize_entry("attributes", &self.attributes)?;
        map.serialize_entry("children", &self.children)?;
        map.end()
    }
}

#[derive(Default)]
struct MJAccordionVisitor;

impl<'de> Visitor<'de> for MJAccordionVisitor {
    type Value = MJAccordion;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an map with properties type, attributes and children")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut result = MJAccordion::default();
        while let Some(key) = access.next_key::<String>()? {
            if key == "type" {
                if access.next_value::<String>()? != NAME {
                    return Err(M::Error::custom(format!("expected type to equal {}", NAME)));
                }
            } else if key == "attributes" {
                result.attributes = access.next_value()?;
            } else if key == "children" {
                result.children = access.next_value()?;
            } else {
                return Err(M::Error::unknown_field(&key, &FIELDS));
            }
        }
        Ok(result)
    }
}

impl<'de> Deserialize<'de> for MJAccordion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MJAccordionVisitor::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_accordion::MJAccordion;

    #[test]
    fn serialize() {
        let elt = MJAccordion::default();
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-accordion","attributes":{},"children":[]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-accordion","attributes":{"margin":"42px","text-align":"left"},"children":[{"type":"mj-accordion-element","attributes":{},"children":[]}]}"#;
        let res: MJAccordion = serde_json::from_str(&json).unwrap();
        assert_eq!(res.attributes.len(), 2);
        assert_eq!(res.children.len(), 1);
    }
}
