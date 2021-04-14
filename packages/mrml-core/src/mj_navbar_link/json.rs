use super::{MJNavbarLink, NAME};
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

const FIELDS: [&str; 3] = ["type", "attributes", "children"];

impl Serialize for MJNavbarLink {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("type", NAME)?;
        map.serialize_entry("attributes", &self.attributes)?;
        map.serialize_entry("children", &self.children)?;
        map.end()
    }
}

#[derive(Default)]
struct MJNavbarLinkVisitor;

impl<'de> Visitor<'de> for MJNavbarLinkVisitor {
    type Value = MJNavbarLink;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an map with properties type, attributes and children")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut result = MJNavbarLink::default();
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

impl<'de> Deserialize<'de> for MJNavbarLink {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MJNavbarLinkVisitor::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_navbar_link::MJNavbarLink;

    #[test]
    fn serialize() {
        let mut elt = MJNavbarLink::default();
        elt.attributes
            .insert("href".into(), "https://jolimail.io".into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-navbar-link","attributes":{"href":"https://jolimail.io"},"children":[]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-navbar-link","attributes":{"margin-bottom":"20px"},"children":["Hello",{"type":"comment","children":"World"}]}"#;
        let res: MJNavbarLink = serde_json::from_str(&json).unwrap();
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.children.len(), 2);
    }
}
