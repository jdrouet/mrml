use super::{MJFont, MJFontAttributes, NAME};
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

const FIELDS: [&str; 2] = ["type", "attributes"];

impl Serialize for MJFont {
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
struct MJFontVisitor;

impl<'de> Visitor<'de> for MJFontVisitor {
    type Value = MJFont;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an map with properties type and attributes")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut elt = MJFont::default();
        while let Some(key) = access.next_key::<String>()? {
            if key == "type" {
                if access.next_value::<String>()? != NAME {
                    return Err(M::Error::custom(format!("expected type to equal {}", NAME)));
                }
            } else if key == "attributes" {
                elt.attributes = access.next_value::<MJFontAttributes>()?;
            } else {
                return Err(M::Error::unknown_field(&key, &FIELDS));
            }
        }
        Ok(elt)
    }
}

impl<'de> Deserialize<'de> for MJFont {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MJFontVisitor::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_font::{MJFont, MJFontAttributes};

    #[test]
    fn serialize() {
        let elt = MJFont {
            attributes: MJFontAttributes {
                name: "Comic".to_string(),
                href: "somewhere".to_string(),
            },
        };
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-font","attributes":{"name":"Comic","href":"somewhere"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let elt = MJFont {
            attributes: MJFontAttributes {
                name: "Comic".to_string(),
                href: "somewhere".to_string(),
            },
        };
        let json = serde_json::to_string(&elt).unwrap();
        let res: MJFont = serde_json::from_str(&json).unwrap();
        assert_eq!(res.name(), elt.name());
        assert_eq!(res.href(), elt.href());
    }

    #[test]
    fn deserialize_missing_field() {
        let json = r#"{"type":"mj-font","attributes":{"name":"Comic"}}"#.to_string();
        assert!(serde_json::from_str::<MJFont>(&json).is_err());
    }
}
