use super::{MJAttributes, NAME};
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

const FIELDS: [&str; 2] = ["type", "children"];

impl Serialize for MJAttributes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("type", NAME)?;
        if !self.children.is_empty() {
            map.serialize_entry("children", &self.children)?;
        }
        map.end()
    }
}

#[derive(Default)]
struct MJAttributesVisitor;

impl<'de> Visitor<'de> for MJAttributesVisitor {
    type Value = MJAttributes;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an map with properties type and children")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut result = MJAttributes::default();
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

impl<'de> Deserialize<'de> for MJAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MJAttributesVisitor::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_attributes::MJAttributes;
    use crate::mj_attributes_all::MJAttributesAll;
    use crate::mj_attributes_class::MJAttributesClass;

    #[test]
    fn serialize() {
        let mut elt = MJAttributes::default();
        elt.children.push(MJAttributesAll::default().into());
        elt.children
            .push(MJAttributesClass::new("name".into()).into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-attributes","children":[{"type":"mj-all"},{"type":"mj-class","name":"name"}]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-attributes","children":[{"type":"mj-all"},{"type":"mj-class","name":"name"}]}"#;
        let res: MJAttributes = serde_json::from_str(json).unwrap();
        assert_eq!(res.children.len(), 2);
        let next = serde_json::to_string(&res).unwrap();
        assert_eq!(next, json);
    }
}
