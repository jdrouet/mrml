use super::{MJMLChild, MJMLChildren, MJML, NAME};
use crate::json_children_serializer;
use serde::de::{Error, MapAccess, SeqAccess, Visitor};
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

const FIELDS: [&str; 3] = ["type", "attributes", "children"];

impl Serialize for MJMLChildren {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_seq(Some(2))?;
        if let Some(ref head) = self.head {
            map.serialize_element(head)?;
        }
        if let Some(ref body) = self.body {
            map.serialize_element(body)?;
        }
        map.end()
    }
}

#[derive(Default)]
struct MJMLChildrenVisitor;

impl<'de> Visitor<'de> for MJMLChildrenVisitor {
    type Value = MJMLChildren;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence with title and text elements")
    }

    fn visit_seq<S>(self, mut access: S) -> Result<Self::Value, S::Error>
    where
        S: SeqAccess<'de>,
    {
        let mut result = MJMLChildren::default();
        while let Some(value) = access.next_element::<MJMLChild>()? {
            match value {
                MJMLChild::MJHead(head) => result.head = Some(head),
                MJMLChild::MJBody(body) => result.body = Some(body),
            };
        }
        Ok(result)
    }
}

impl<'de> Deserialize<'de> for MJMLChildren {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(MJMLChildrenVisitor::default())
    }
}

json_children_serializer!(MJML, NAME);

#[derive(Default)]
struct MJMLVisitor;

impl<'de> Visitor<'de> for MJMLVisitor {
    type Value = MJML;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an map with properties type, attributes and children")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut result = MJML::default();
        while let Some(key) = access.next_key::<String>()? {
            if key == "type" {
                if access.next_value::<String>()? != NAME {
                    return Err(M::Error::custom(format!("expected type to equal {}", NAME)));
                }
            } else if key == "children" {
                result.children = access.next_value()?;
            } else {
                return Err(M::Error::unknown_field(&key, &FIELDS));
            }
        }
        Ok(result)
    }
}

impl<'de> Deserialize<'de> for MJML {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MJMLVisitor::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::mjml::MJML;

    #[test]
    fn serialize() {
        let elt = MJML::default();
        assert_eq!(serde_json::to_string(&elt).unwrap(), r#"{"type":"mjml"}"#);
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mjml","children":[{"type":"mj-head"},{"type":"mj-body","children":["Hello World!"]}]}"#;
        let res: MJML = serde_json::from_str(json).unwrap();
        assert!(res.children.head.is_some());
        assert!(res.children.body.is_some());
        let next = serde_json::to_string(&res).unwrap();
        assert_eq!(next, json);
    }
}
