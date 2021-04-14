use super::{MJCarousel, NAME};
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

const FIELDS: [&str; 2] = ["type", "attributes"];

impl Serialize for MJCarousel {
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
struct MJCarouselVisitor;

impl<'de> Visitor<'de> for MJCarouselVisitor {
    type Value = MJCarousel;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an map with properties type and attributes")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut result = MJCarousel::default();
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

impl<'de> Deserialize<'de> for MJCarousel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MJCarouselVisitor::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_carousel::MJCarousel;

    #[test]
    fn serialize() {
        let mut elt = MJCarousel::default();
        elt.attributes.insert("margin".into(), "42px".into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-carousel","attributes":{"margin":"42px"},"children":[]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-carousel","attributes":{"margin":"42px"},"children":[{"type":"mj-carousel-image","attributes":{"src":"https://jolimail.io"}}]}"#;
        let res: MJCarousel = serde_json::from_str(json).unwrap();
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.children.len(), 1);
        let next = serde_json::to_string(&res).unwrap();
        assert_eq!(next, json);
    }
}
