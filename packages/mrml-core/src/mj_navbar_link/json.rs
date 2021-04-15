use super::{MJNavbarLink, NAME};
use crate::json_attrs_and_children_deserializer;
use crate::json_attrs_and_children_serializer;
use serde::de::{Error, MapAccess};
use serde::ser::SerializeMap;
use std::fmt;

json_attrs_and_children_serializer!(MJNavbarLink, NAME);
json_attrs_and_children_deserializer!(MJNavbarLink, MJNavbarLinkVisitor, NAME);

#[cfg(test)]
mod tests {
    use crate::mj_navbar_link::MJNavbarLink;

    #[test]
    fn serialize() {
        let mut elt = MJNavbarLink::default();
        elt.attributes
            .insert("href".into(), "https://jolimail.io".into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-navbar-link","attributes":{"href":"https://jolimail.io"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-navbar-link","attributes":{"margin-bottom":"20px"},"children":["Hello",{"type":"comment","children":"World"}]}"#;
        let res: MJNavbarLink = serde_json::from_str(&json).unwrap();
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.children.len(), 2);
    }
}
