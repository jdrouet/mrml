use super::{MJAccordionTitle, NAME};
use crate::json_attrs_and_children_deserializer;
use crate::json_attrs_and_children_serializer;
use serde::de::{Error, MapAccess};
use serde::ser::SerializeMap;
use std::fmt;

json_attrs_and_children_serializer!(MJAccordionTitle, NAME);
json_attrs_and_children_deserializer!(MJAccordionTitle, MJAccordionTitleVisitor, NAME);

#[cfg(test)]
mod tests {
    use crate::mj_accordion_title::MJAccordionTitle;
    use crate::text::Text;

    #[test]
    fn serialize() {
        let mut elt = MJAccordionTitle::default();
        elt.attributes
            .insert("margin".to_string(), "12px".to_string());
        elt.children.push(Text::from("Hello").into());
        elt.children.push(Text::from("World").into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-accordion-title","attributes":{"margin":"12px"},"children":["Hello","World"]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-accordion-title","attributes":{"margin":"12px"},"children":["Hello","World"]}"#;
        let res: MJAccordionTitle = serde_json::from_str(json).unwrap();
        assert_eq!(res.children.len(), 2);
        let next = serde_json::to_string(&res).unwrap();
        assert_eq!(next, json);
    }
}
