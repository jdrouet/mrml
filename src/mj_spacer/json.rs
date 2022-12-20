use super::{MJSpacer, NAME};
use crate::json_attrs_deserializer;
use crate::json_attrs_serializer;
use serde::de::{Error, MapAccess};
use serde::ser::SerializeMap;
use std::fmt;

json_attrs_serializer!(MJSpacer, NAME);
json_attrs_deserializer!(MJSpacer, MJSpacerVisitor, NAME);

#[cfg(test)]
mod tests {
    use crate::mj_spacer::MJSpacer;

    #[test]
    fn serialize() {
        let mut elt = MJSpacer::default();
        elt.attributes.insert("margin-bottom".into(), "20px".into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-spacer","attributes":{"margin-bottom":"20px"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let elt = MJSpacer::default();
        let json = serde_json::to_string(&elt).unwrap();
        let _res: MJSpacer = serde_json::from_str(&json).unwrap();
    }
}
