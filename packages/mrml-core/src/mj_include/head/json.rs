use crate::prelude::json::ComponentAttributes;

impl ComponentAttributes for super::MjIncludeHeadAttributes {
    fn has_attributes(&self) -> bool {
        true
    }

    fn try_from_serde<Err: serde::de::Error>(this: Option<Self>) -> Result<Self, Err>
    where
        Self: Sized,
    {
        this.ok_or_else(|| serde::de::Error::missing_field("attributes"))
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_include::head::{
        MjIncludeHead, MjIncludeHeadAttributes, MjIncludeHeadChild, MjIncludeHeadKind,
    };
    use crate::mj_title::MjTitle;
    use crate::text::Text;

    #[test]
    fn serialize_mjml() {
        let elt = MjIncludeHead::new(
            MjIncludeHeadAttributes::new("memory:include.mjml"),
            vec![MjIncludeHeadChild::MjTitle(MjTitle::from(
                "Hello World!".to_owned(),
            ))],
        );
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-include","attributes":{"path":"memory:include.mjml"},"children":[{"type":"mj-title","children":"Hello World!"}]}"#
        );
    }

    #[test]
    fn serialize_html() {
        let elt = MjIncludeHead::new(
            MjIncludeHeadAttributes::new("memory:include.html").with_kind(MjIncludeHeadKind::Html),
            vec![MjIncludeHeadChild::MjTitle(MjTitle::from(
                "Hello World!".to_owned(),
            ))],
        );
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-include","attributes":{"path":"memory:include.html","type":"html"},"children":[{"type":"mj-title","children":"Hello World!"}]}"#
        );
    }

    #[test]
    fn serialize_css() {
        let elt = MjIncludeHead::new(
            MjIncludeHeadAttributes::new("memory:include.css")
                .with_kind(MjIncludeHeadKind::Css { inline: false }),
            vec![MjIncludeHeadChild::Text(Text::from(
                "* { background-color: red; }",
            ))],
        );
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-include","attributes":{"path":"memory:include.css","type":{"css":{"inline":false}}},"children":["* { background-color: red; }"]}"#
        );
    }

    #[test]
    fn serialize_css_inline() {
        let elt = MjIncludeHead::new(
            MjIncludeHeadAttributes::new("memory:include.css")
                .with_kind(MjIncludeHeadKind::Css { inline: true }),
            vec![MjIncludeHeadChild::Text(Text::from(
                "* { background-color: red; }",
            ))],
        );
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-include","attributes":{"path":"memory:include.css","type":{"css":{"inline":true}}},"children":["* { background-color: red; }"]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-include","attributes":{"path":"memory:include.mjml"},"children":[{"type":"mj-title","children":"Hello World!"}]}"#;
        let res: MjIncludeHead = serde_json::from_str(json).unwrap();
        assert_eq!(res.0.attributes.path, "memory:include.mjml");
        assert!(!res.0.children.is_empty());
    }
}
