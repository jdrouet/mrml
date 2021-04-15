use super::{MJTitle, NAME};
use crate::json_children_deserializer;
use crate::json_children_serializer;
use serde::de::{Error, MapAccess};
use serde::ser::SerializeMap;
use std::fmt;

json_children_serializer!(MJTitle, NAME);
json_children_deserializer!(MJTitle, MJTitleVisitor, NAME);

#[cfg(test)]
mod tests {
    use crate::mj_title::MJTitle;

    #[test]
    fn serialize() {
        let elt = MJTitle::from("Hello World");
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-title","children":"Hello World"}"#
        );
    }

    #[test]
    fn deserialize() {
        let elt = MJTitle::from("Hello World");
        let json = serde_json::to_string(&elt).unwrap();
        let res: MJTitle = serde_json::from_str(&json).unwrap();
        assert_eq!(res.children, elt.children);
    }
}
