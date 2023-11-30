#[cfg(test)]
mod tests {
    use crate::mj_include::head::{MjIncludeHead, MjIncludeHeadChild, MjIncludeHeadKind};
    use crate::mj_title::MjTitle;
    use crate::text::Text;

    #[test]
    fn serialize_mjml() {
        let mut elt = MjIncludeHead::default();
        elt.attributes.path = "memory:include.mjml".to_string();
        elt.children = vec![MjIncludeHeadChild::MjTitle(MjTitle::from(
            "Hello World!".to_owned(),
        ))];
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-include","attributes":{"path":"memory:include.mjml"},"children":[{"type":"mj-title","children":"Hello World!"}]}"#
        );
    }

    #[test]
    fn serialize_html() {
        let mut elt = MjIncludeHead::default();
        elt.attributes.path = "memory:include.html".to_string();
        elt.attributes.kind = MjIncludeHeadKind::Html;
        elt.children = vec![MjIncludeHeadChild::MjTitle(MjTitle::from(
            "Hello World!".to_owned(),
        ))];
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-include","attributes":{"path":"memory:include.html","type":"html"},"children":[{"type":"mj-title","children":"Hello World!"}]}"#
        );
    }

    #[test]
    fn serialize_css() {
        let mut elt = MjIncludeHead::default();
        elt.attributes.path = "memory:include.css".to_string();
        elt.attributes.kind = MjIncludeHeadKind::Css { inline: false };
        elt.children = vec![MjIncludeHeadChild::Text(Text::from(
            "* { background-color: red; }",
        ))];
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-include","attributes":{"path":"memory:include.css","type":{"css":{"inline":false}}},"children":["* { background-color: red; }"]}"#
        );
    }

    #[test]
    fn serialize_css_inline() {
        let mut elt = MjIncludeHead::default();
        elt.attributes.path = "memory:include.css".to_string();
        elt.attributes.kind = MjIncludeHeadKind::Css { inline: true };
        elt.children = vec![MjIncludeHeadChild::Text(Text::from(
            "* { background-color: red; }",
        ))];
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-include","attributes":{"path":"memory:include.css","type":{"css":{"inline":true}}},"children":["* { background-color: red; }"]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-include","attributes":{"path":"memory:include.mjml"},"children":[{"type":"mj-title","children":"Hello World!"}]}"#;
        let res: MjIncludeHead = serde_json::from_str(json).unwrap();
        assert_eq!(res.attributes.path, "memory:include.mjml");
        assert!(!res.children.is_empty());
    }
}
