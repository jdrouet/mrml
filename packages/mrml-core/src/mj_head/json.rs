use super::{MJHead, NAME};
use crate::json_children_deserializer;
use crate::json_children_serializer;
use serde::de::{Error, MapAccess};
use serde::ser::SerializeMap;
use std::fmt;

json_children_serializer!(MJHead, NAME);
json_children_deserializer!(MJHead, MJHeadVisitor, NAME);

#[cfg(test)]
mod tests {
    use crate::mj_head::MJHead;

    #[test]
    fn serialize() {
        let elt = MJHead::default();
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-head"}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-head","children":[{"type":"mj-font","attributes":{"name":"Comic","href":"http://jolimail.io"}},{"type":"mj-breakpoint","attributes":{"width":"12px"}}]}"#;
        let res: MJHead = serde_json::from_str(json).unwrap();
        assert_eq!(res.children.len(), 2);
        let next = serde_json::to_string(&res).unwrap();
        assert_eq!(next, json);
    }
}
