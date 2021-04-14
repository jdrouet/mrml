use super::{MJImage, NAME};
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt;

const FIELDS: [&str; 2] = ["type", "attributes"];

impl Serialize for MJImage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("type", NAME)?;
        map.serialize_entry("attributes", &self.attributes)?;
        map.end()
    }
}

#[derive(Default)]
struct MJImageVisitor;

impl<'de> Visitor<'de> for MJImageVisitor {
    type Value = MJImage;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an map with properties type and attributes")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut elt = MJImage::default();
        while let Some(key) = access.next_key::<String>()? {
            if key == "type" {
                if access.next_value::<String>()? != NAME {
                    return Err(M::Error::custom(format!("expected type to equal {}", NAME)));
                }
            } else if key == "attributes" {
                elt.attributes = access.next_value::<HashMap<String, String>>()?;
            } else {
                return Err(M::Error::unknown_field(&key, &FIELDS));
            }
        }
        Ok(elt)
    }
}

impl<'de> Deserialize<'de> for MJImage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MJImageVisitor::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_image::MJImage;
    use std::collections::HashMap;

    #[test]
    fn serialize() {
        let mut attrs = HashMap::new();
        attrs.insert("href".to_string(), "https://jolimail.io".to_string());
        let elt = MJImage { attributes: attrs };
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-image","attributes":{"href":"https://jolimail.io"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-image","attributes":{"href":"https://jolimail.io"}}"#;
        let res: MJImage = serde_json::from_str(&json).unwrap();
        assert_eq!(res.attributes.get("href").unwrap(), "https://jolimail.io");
    }
}
