#[cfg(test)]
mod tests {
    use crate::mj_accordion_title::MjAccordionTitle;
    use crate::text::Text;

    #[test]
    fn serialize() {
        let mut elt = MjAccordionTitle::default();
        elt.attributes
            .insert("margin".to_string(), "12px".to_string());
        elt.children.push(Text::from("Hello"));
        elt.children.push(Text::from("World"));
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-accordion-title","attributes":{"margin":"12px"},"children":["Hello","World"]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-accordion-title","attributes":{"margin":"12px"},"children":["Hello","World"]}"#;
        let res: MjAccordionTitle = serde_json::from_str(json).unwrap();
        assert_eq!(res.children.len(), 2);
        let next = serde_json::to_string(&res).unwrap();
        assert_eq!(next, json);
    }
}
