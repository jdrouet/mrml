#[cfg(test)]
mod tests {
    use crate::mj_html_attributes::{MjHtmlAttributes, MjSelector};
    use crate::mj_selector::MjSelectorAttributes;

    #[test]
    fn serialize() {
        let mut elt = MjHtmlAttributes::default();

        elt.children.push(MjSelector::new(
            MjSelectorAttributes {
                path: ".class".into(),
            },
            Vec::new(),
        ));

        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"children":[{}],"mj-selector":{"attributes":{},"name":"","type":""}}"#
        )
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-html-attributes","children":[{"type":"mj-selector"}, {"type":"mj-html-attribute"}]}"#;
        let res: MjHtmlAttributes = serde_json::from_str(json).unwrap();
        assert_eq!(res.children.len(), 2);
        let next = serde_json::to_string(&res).unwrap();
        assert_eq!(next, json);
    }
}
