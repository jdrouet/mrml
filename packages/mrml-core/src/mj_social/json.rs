use super::{MJSocial, NAME};
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

const FIELDS: [&str; 3] = ["type", "attributes", "children"];

impl Serialize for MJSocial {
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
struct MJSocialVisitor;

impl<'de> Visitor<'de> for MJSocialVisitor {
    type Value = MJSocial;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an map with properties type, attributes and children")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut result = MJSocial::default();
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

impl<'de> Deserialize<'de> for MJSocial {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MJSocialVisitor::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_social::MJSocial;

    #[test]
    fn serialize() {
        let mut elt = MJSocial::default();
        elt.attributes.insert("margin".into(), "42px".into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-social","attributes":{"margin":"42px"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-social","attributes":{"margin":"0px"},"children":[{"type":"mj-social-element","attributes":{"name":"twitter"}},{"type":"comment","children":"World"}]}"#;
        let res: MJSocial = serde_json::from_str(&json).unwrap();
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.children.len(), 2);
    }
}
