use super::{MJAccordionElement, MJAccordionElementChild, MJAccordionElementChildren, NAME};
use crate::json_attrs_and_children_serializer;
use serde::de::{Error, MapAccess, SeqAccess, Visitor};
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

const FIELDS: [&str; 3] = ["type", "attributes", "children"];

impl Serialize for MJAccordionElementChildren {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_seq(Some(2))?;
        if let Some(ref title) = self.title {
            map.serialize_element(title)?;
        }
        if let Some(ref text) = self.text {
            map.serialize_element(text)?;
        }
        map.end()
    }
}

#[derive(Default)]
struct MJAccordionElementChildrenVisitor;

impl<'de> Visitor<'de> for MJAccordionElementChildrenVisitor {
    type Value = MJAccordionElementChildren;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence with title and text elements")
    }

    fn visit_seq<S>(self, mut access: S) -> Result<Self::Value, S::Error>
    where
        S: SeqAccess<'de>,
    {
        let mut result = MJAccordionElementChildren::default();
        while let Some(value) = access.next_element::<MJAccordionElementChild>()? {
            match value {
                MJAccordionElementChild::MJAccordionTitle(title) => result.title = Some(title),
                MJAccordionElementChild::MJAccordionText(text) => result.text = Some(text),
                _ => (),
            };
        }
        Ok(result)
    }
}

impl<'de> Deserialize<'de> for MJAccordionElementChildren {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(MJAccordionElementChildrenVisitor::default())
    }
}

json_attrs_and_children_serializer!(MJAccordionElement, NAME);

#[derive(Default)]
struct MJAccordionElementVisitor;

impl<'de> Visitor<'de> for MJAccordionElementVisitor {
    type Value = MJAccordionElement;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an map with properties type, attributes and children")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut result = MJAccordionElement::default();
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

impl<'de> Deserialize<'de> for MJAccordionElement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MJAccordionElementVisitor::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_accordion_element::MJAccordionElement;

    #[test]
    fn serialize() {
        let mut elt = MJAccordionElement::default();
        elt.attributes
            .insert("margin".to_string(), "12px".to_string());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-accordion-element","attributes":{"margin":"12px"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-accordion-element","attributes":{"margin":"12px"},"children":[{"type":"mj-accordion-title"},{"type":"mj-accordion-text"}]}"#;
        let res: MJAccordionElement = serde_json::from_str(json).unwrap();
        assert!(res.children.text.is_some());
        assert!(res.children.title.is_some());
        let next = serde_json::to_string(&res).unwrap();
        assert_eq!(next, json);
    }
}
