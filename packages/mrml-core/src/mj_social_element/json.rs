use super::{MJSocialElement, NAME};
use crate::json_attrs_and_children_deserializer;
use crate::json_attrs_and_children_serializer;
use serde::de::{Error, MapAccess};
use serde::ser::SerializeMap;
use std::fmt;

json_attrs_and_children_serializer!(MJSocialElement, NAME);
json_attrs_and_children_deserializer!(MJSocialElement, MJSocialElementVisitor, NAME);

#[cfg(test)]
mod tests {
    use crate::mj_social_element::MJSocialElement;

    #[test]
    fn serialize() {
        let mut elt = MJSocialElement::default();
        elt.attributes.insert("name".into(), "twitter".into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-social-element","attributes":{"name":"twitter"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-social-element","attributes":{"margin-bottom":"20px","name":"twitter"},"children":["Hello",{"type":"comment","children":"World"}]}"#;
        let res: MJSocialElement = serde_json::from_str(&json).unwrap();
        assert_eq!(res.attributes.len(), 2);
        assert_eq!(res.children.len(), 2);
    }
}
