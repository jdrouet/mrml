use super::{MJNavbar, NAME};
use crate::json_attrs_and_children_deserializer;
use crate::json_attrs_and_children_serializer;
use serde::de::{Error, MapAccess};
use serde::ser::SerializeMap;
use std::fmt;

json_attrs_and_children_serializer!(MJNavbar, NAME);
json_attrs_and_children_deserializer!(MJNavbar, MJNavbarVisitor, NAME);

#[cfg(test)]
mod tests {
    use crate::mj_navbar::MJNavbar;

    #[test]
    fn serialize() {
        let mut elt = MJNavbar::default();
        elt.attributes.insert("margin".into(), "42px".into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-navbar","attributes":{"margin":"42px"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-navbar","attributes":{"margin-bottom":"20px"},"children":[{"type":"mj-navbar-link"},{"type":"comment","children":"World"}]}"#;
        let res: MJNavbar = serde_json::from_str(&json).unwrap();
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.children.len(), 2);
    }
}
