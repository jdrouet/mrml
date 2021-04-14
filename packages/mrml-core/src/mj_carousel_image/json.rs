use super::{MJCarouselImage, NAME};
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

const FIELDS: [&str; 2] = ["type", "attributes"];

impl Serialize for MJCarouselImage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("type", NAME)?;
        if !self.attributes.is_empty() {
            map.serialize_entry("attributes", &self.attributes)?;
        }
        map.end()
    }
}

#[derive(Default)]
struct MJCarouselImageVisitor;

impl<'de> Visitor<'de> for MJCarouselImageVisitor {
    type Value = MJCarouselImage;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an map with properties type and attributes")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut result = MJCarouselImage::default();
        while let Some(key) = access.next_key::<String>()? {
            if key == "type" {
                if access.next_value::<String>()? != NAME {
                    return Err(M::Error::custom(format!("expected type to equal {}", NAME)));
                }
            } else if key == "attributes" {
                result.attributes = access.next_value()?;
            } else {
                return Err(M::Error::unknown_field(&key, &FIELDS));
            }
        }
        Ok(result)
    }
}

impl<'de> Deserialize<'de> for MJCarouselImage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MJCarouselImageVisitor::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_carousel_image::MJCarouselImage;

    #[test]
    fn serialize() {
        let mut elt = MJCarouselImage::default();
        elt.attributes
            .insert("src".into(), "https://jolimail.io".into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-carousel-image","attributes":{"src":"https://jolimail.io"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-carousel-image","attributes":{"src":"https://jolimail.io"}}"#;
        let res: MJCarouselImage = serde_json::from_str(json).unwrap();
        assert_eq!(res.attributes.len(), 1);
        let next = serde_json::to_string(&res).unwrap();
        assert_eq!(next, json);
    }
}
