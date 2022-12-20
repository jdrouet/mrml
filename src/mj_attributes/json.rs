use super::{MJAttributes, NAME};
use crate::json_children_deserializer;
use crate::json_children_serializer;
use serde::de::{Error, MapAccess};
use serde::ser::SerializeMap;
use std::fmt;

json_children_serializer!(MJAttributes, NAME);
json_children_deserializer!(MJAttributes, MJAttributesVisitor, NAME);

#[cfg(test)]
mod tests {
    use crate::mj_attributes::MJAttributes;
    use crate::mj_attributes_all::MJAttributesAll;
    use crate::mj_attributes_class::MJAttributesClass;

    #[test]
    fn serialize() {
        let mut elt = MJAttributes::default();
        elt.children.push(MJAttributesAll::default().into());
        elt.children
            .push(MJAttributesClass::new("name".into()).into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-attributes","children":[{"type":"mj-all"},{"type":"mj-class","name":"name"}]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-attributes","children":[{"type":"mj-all"},{"type":"mj-class","name":"name"}]}"#;
        let res: MJAttributes = serde_json::from_str(json).unwrap();
        assert_eq!(res.children.len(), 2);
        let next = serde_json::to_string(&res).unwrap();
        assert_eq!(next, json);
    }
}
