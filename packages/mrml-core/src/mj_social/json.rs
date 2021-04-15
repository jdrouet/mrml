use super::{MJSocial, NAME};
use crate::json_attrs_and_children_deserializer;
use crate::json_attrs_and_children_serializer;
use serde::de::{Error, MapAccess};
use serde::ser::SerializeMap;
use std::fmt;

json_attrs_and_children_serializer!(MJSocial, NAME);
json_attrs_and_children_deserializer!(MJSocial, MJSocialVisitor, NAME);

#[cfg(test)]
mod tests {
    use crate::mj_social::MJSocial;

    #[test]
    fn serialize() {
        let mut elt = MJSocial::default();
        elt.attributes.insert("margin".into(), "42px".into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-social","attributes":{"margin":"42px"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-social","attributes":{"margin":"0px"},"children":[{"type":"mj-social-element","attributes":{"name":"twitter"}},{"type":"comment","children":"World"}]}"#;
        let res: MJSocial = serde_json::from_str(&json).unwrap();
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.children.len(), 2);
    }
}
