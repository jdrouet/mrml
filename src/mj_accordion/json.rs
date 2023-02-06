#[cfg(test)]
mod tests {
    use crate::mj_accordion::MjAccordion;
    use crate::mj_accordion_element::MjAccordionElement;

    #[test]
    fn serialize() {
        let mut elt = MjAccordion::default();
        elt.attributes.insert("margin".into(), "42px".into());
        elt.children.push(MjAccordionElement::default().into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-accordion","attributes":{"margin":"42px"},"children":[{"type":"mj-accordion-element"}]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-accordion","attributes":{"margin":"42px","text-align":"left"},"children":[{"type":"mj-accordion-element"}]}"#;
        let res: MjAccordion = serde_json::from_str(json).unwrap();
        assert_eq!(res.attributes.len(), 2);
        assert_eq!(res.children.len(), 1);
    }
}
