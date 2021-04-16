use super::{MJFont, NAME};
use crate::json_attrs_deserializer;
use crate::json_attrs_serializer;
use serde::de::{Error, MapAccess};
use serde::ser::SerializeMap;
use std::fmt;

json_attrs_serializer!(MJFont, NAME);
json_attrs_deserializer!(MJFont, MJFontVisitor, NAME);

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
