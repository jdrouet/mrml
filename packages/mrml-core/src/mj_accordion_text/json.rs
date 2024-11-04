#[cfg(test)]
mod tests {
    use crate::mj_accordion_text::{MjAccordionText, MjRawChild};
    use crate::text::Text;

    #[test]
    fn serialize() {
        let mut elt = MjAccordionText::default();
        elt.attributes
            .insert("margin".to_string(), Some("12px".to_string()));
        elt.children.push(MjRawChild::Text(Text::from("Hello")));
        elt.children.push(MjRawChild::Text(Text::from("World")));
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-accordion-text","attributes":{"margin":"12px"},"children":["Hello","World"]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-accordion-text","attributes":{"margin":"12px"},"children":["Hello","World"]}"#;
        let res: MjAccordionText = serde_json::from_str(json).unwrap();
        assert_eq!(res.children.len(), 2);
        let next = serde_json::to_string(&res).unwrap();
        assert_eq!(next, json);
    }
}
