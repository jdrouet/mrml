use super::{MJHead, NAME};
use crate::json_children_serializer;
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer};
use std::fmt;

const FIELDS: [&str; 2] = ["type", "children"];

json_children_serializer!(MJHead, NAME);

#[derive(Default)]
struct MJHeadVisitor;

impl<'de> Visitor<'de> for MJHeadVisitor {
    type Value = MJHead;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an map with properties type and children")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut result = MJHead::default();
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

impl<'de> Deserialize<'de> for MJHead {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MJHeadVisitor::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_head::MJHead;

    #[test]
    fn serialize() {
        let elt = MJHead::default();
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-head"}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-head","children":[{"type":"mj-font","attributes":{"name":"Comic","href":"http://jolimail.io"}},{"type":"mj-breakpoint","attributes":{"value":"12px"}}]}"#;
        let res: MJHead = serde_json::from_str(json).unwrap();
        assert_eq!(res.children.len(), 2);
        let next = serde_json::to_string(&res).unwrap();
        assert_eq!(next, json);
    }
}
