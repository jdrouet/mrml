use super::{MJRaw, NAME};
use crate::json_children_serializer;
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer};
use std::fmt;

const FIELDS: [&str; 2] = ["type", "children"];

json_children_serializer!(MJRaw, NAME);

#[derive(Default)]
struct MJRawVisitor;

impl<'de> Visitor<'de> for MJRawVisitor {
    type Value = MJRaw;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an map with properties type and children")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut result = MJRaw::default();
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

impl<'de> Deserialize<'de> for MJRaw {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MJRawVisitor::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_raw::MJRaw;
    use crate::text::Text;

    #[test]
    fn serialize() {
        let mut elt = MJRaw::default();
        elt.children.push(Text::from("Hello").into());
        elt.children.push(Text::from("World").into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-raw","children":["Hello","World"]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-raw","children":["Hello","World"]}"#;
        let res: MJRaw = serde_json::from_str(json).unwrap();
        assert_eq!(res.children.len(), 2);
        let next = serde_json::to_string(&res).unwrap();
        assert_eq!(next, json);
    }
}
