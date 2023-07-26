#[cfg(test)]
mod tests {
    use crate::mj_button::MjButton;
    use crate::mj_include::body::{MjIncludeBody, MjIncludeBodyChild, MjIncludeBodyKind};

    #[test]
    fn serialize_mjml() {
        let mut elt = MjIncludeBody::default();
        elt.attributes.path = "memory:include.mjml".to_string();
        elt.children = vec![MjIncludeBodyChild::MjButton(MjButton::default())];
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-include","attributes":{"path":"memory:include.mjml"},"children":[{"type":"mj-button"}]}"#
        );
    }

    #[test]
    fn serialize_html() {
        let mut elt = MjIncludeBody::default();
        elt.attributes.path = "memory:include.html".to_string();
        elt.attributes.kind = MjIncludeBodyKind::Html;
        elt.children = vec![MjIncludeBodyChild::MjButton(MjButton::default())];
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-include","attributes":{"path":"memory:include.html","type":"html"},"children":[{"type":"mj-button"}]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-include","attributes":{"path":"memory:include.mjml"},"children":[{"type":"mj-button"}]}"#;
        let res: MjIncludeBody = serde_json::from_str(json).unwrap();
        assert_eq!(res.attributes.path, "memory:include.mjml");
        assert!(!res.children.is_empty());
    }
}
