use super::{MJAccordionTitle, NAME};
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

const FIELDS: [&str; 3] = ["type", "attributes", "children"];

impl Serialize for MJAccordionTitle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("type", NAME)?;
        if !self.attributes.is_empty() {
            map.serialize_entry("attributes", &self.attributes)?;
        }
        if !self.children.is_empty() {
            map.serialize_entry("children", &self.children)?;
        }
        map.end()
    }
}

#[derive(Default)]
struct MJAccordionTitleVisitor;

impl<'de> Visitor<'de> for MJAccordionTitleVisitor {
    type Value = MJAccordionTitle;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an map with properties type, attributes and children")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut result = MJAccordionTitle::default();
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

impl<'de> Deserialize<'de> for MJAccordionTitle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MJAccordionTitleVisitor::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_accordion_title::MJAccordionTitle;
    use crate::text::Text;

    #[test]
    fn serialize() {
        let mut elt = MJAccordionTitle::default();
        elt.attributes
            .insert("margin".to_string(), "12px".to_string());
        elt.children.push(Text::from("Hello").into());
        elt.children.push(Text::from("World").into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-accordion-title","attributes":{"margin":"12px"},"children":["Hello","World"]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-accordion-title","attributes":{"margin":"12px"},"children":["Hello","World"]}"#;
        let res: MJAccordionTitle = serde_json::from_str(json).unwrap();
        assert_eq!(res.children.len(), 2);
        let next = serde_json::to_string(&res).unwrap();
        assert_eq!(next, json);
    }
}
