use super::{MJBody, NAME};
use crate::json_attrs_and_children_deserializer;
use crate::json_attrs_and_children_serializer;
use serde::de::{Error, MapAccess};
use serde::ser::SerializeMap;
use std::fmt;

json_attrs_and_children_serializer!(MJBody, NAME);
json_attrs_and_children_deserializer!(MJBody, MJBodyVisitor, NAME);

#[cfg(test)]
mod tests {
    use crate::mj_body::MJBody;
    use crate::text::Text;

    #[test]
    fn serialize() {
        let mut elt = MJBody::default();
        elt.attributes.insert("margin".into(), "42px".into());
        elt.children.push(Text::from("Hello World!").into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-body","attributes":{"margin":"42px"},"children":["Hello World!"]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-body","attributes":{"margin-bottom":"20px"},"children":[{"type":"comment","children":"Hello World!"},"Hello World!"]}"#;
        let res: MJBody = serde_json::from_str(json).unwrap();
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.children.len(), 2);
    }
}
