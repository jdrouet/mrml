use super::{MJAttributesAll, NAME};
use crate::json_attrs_deserializer;
use crate::json_attrs_serializer;
use serde::de::{Error, MapAccess};
use serde::ser::SerializeMap;
use std::fmt;

json_attrs_serializer!(MJAttributesAll, NAME);
json_attrs_deserializer!(MJAttributesAll, MJAttributesAllVisitor, NAME);

#[cfg(test)]
mod tests {
    use crate::mj_attributes_all::MJAttributesAll;

    #[test]
    fn serialize() {
        let mut elt = MJAttributesAll::default();
        elt.attributes.insert("margin-bottom".into(), "20px".into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-all","attributes":{"margin-bottom":"20px"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let elt = MJAttributesAll::default();
        let json = serde_json::to_string(&elt).unwrap();
        let _res: MJAttributesAll = serde_json::from_str(&json).unwrap();
    }
}
