use super::{MJButton, NAME};
use crate::json_attrs_and_children_deserializer;
use crate::json_attrs_and_children_serializer;
use serde::de::{Error, MapAccess};
use serde::ser::SerializeMap;
use std::fmt;

json_attrs_and_children_serializer!(MJButton, NAME);
json_attrs_and_children_deserializer!(MJButton, MJButtonVisitor, NAME);

#[cfg(test)]
mod tests {
    use crate::mj_button::MJButton;
    use crate::text::Text;

    #[test]
    fn serialize() {
        let mut elt = MJButton::default();
        elt.attributes.insert("margin".into(), "42px".into());
        elt.children.push(Text::from("Hello World!").into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-button","attributes":{"margin":"42px"},"children":["Hello World!"]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-button","attributes":{"margin-bottom":"20px"},"children":[{"type":"comment","children":"Hello World!"},"Hello World!"]}"#;
        let res: MJButton = serde_json::from_str(json).unwrap();
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.children.len(), 2);
    }
}
