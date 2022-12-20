use super::{MJPreview, NAME};
use crate::json_children_deserializer;
use crate::json_children_serializer;
use serde::de::{Error, MapAccess};
use serde::ser::SerializeMap;
use std::fmt;

json_children_serializer!(MJPreview, NAME);
json_children_deserializer!(MJPreview, MJPreviewVisitor, NAME);

#[cfg(test)]
mod tests {
    use crate::mj_preview::MJPreview;

    #[test]
    fn serialize() {
        let elt = MJPreview::from("Hello World");
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-preview","children":"Hello World"}"#
        );
    }

    #[test]
    fn deserialize() {
        let elt = MJPreview::from("Hello World");
        let json = serde_json::to_string(&elt).unwrap();
        let res: MJPreview = serde_json::from_str(&json).unwrap();
        assert_eq!(res.children, elt.children);
    }
}
