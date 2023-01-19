#[cfg(test)]
mod tests {
    use crate::mj_accordion::MJAccordion;

    #[test]
    fn serialize() {
        let elt = MJAccordion::default();
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-accordion"}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-accordion","attributes":{"margin":"42px","text-align":"left"},"children":[{"type":"mj-accordion-element"}]}"#;
        let res: MJAccordion = serde_json::from_str(json).unwrap();
        assert_eq!(res.attributes.len(), 2);
        assert_eq!(res.children.len(), 1);
    }
}
