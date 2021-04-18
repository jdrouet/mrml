use super::{MJStyle, NAME};
use crate::json_attrs_and_children_deserializer;
use crate::json_attrs_and_children_serializer;
use serde::de::{Error, MapAccess};
use serde::ser::SerializeMap;
use std::fmt;

json_attrs_and_children_serializer!(MJStyle, NAME);
json_attrs_and_children_deserializer!(MJStyle, MJStyleVisitor, NAME);

#[cfg(test)]
mod tests {
    use crate::mj_style::MJStyle;

    #[test]
    fn serialize() {
        let elt = MJStyle::from("Hello World");
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-style","children":"Hello World"}"#
        );
    }

    #[test]
    fn deserialize() {
        let elt = MJStyle::from("Hello World");
        let json = serde_json::to_string(&elt).unwrap();
        let res: MJStyle = serde_json::from_str(&json).unwrap();
        assert_eq!(res.children, elt.children);
    }
}
