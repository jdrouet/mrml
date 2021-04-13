use super::{MJPreview, NAME};
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

const FIELDS: [&str; 2] = ["type", "children"];

impl Serialize for MJPreview {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("type", NAME)?;
        map.serialize_entry("children", &self.0)?;
        map.end()
    }
}

#[derive(Default)]
struct MJPreviewVisitor;

impl<'de> Visitor<'de> for MJPreviewVisitor {
    type Value = MJPreview;

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
        Ok(MJPreview(result))
    }
}

impl<'de> Deserialize<'de> for MJPreview {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MJPreviewVisitor::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_preview::MJPreview;

    #[test]
    fn serialize() {
        let elt = MJPreview("Hello World".to_string());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-preview","children":"Hello World"}"#
        );
    }

    #[test]
    fn deserialize() {
        let elt = MJPreview("Hello World".to_string());
        let json = serde_json::to_string(&elt).unwrap();
        let res: MJPreview = serde_json::from_str(&json).unwrap();
        assert_eq!(res.0, elt.0);
    }
}
