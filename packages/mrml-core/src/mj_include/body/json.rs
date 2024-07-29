use crate::prelude::json::JsonAttributes;

impl JsonAttributes for super::MjIncludeBodyAttributes {
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
    use crate::mj_button::MjButton;
    use crate::mj_include::body::{
        MjIncludeBody, MjIncludeBodyAttributes, MjIncludeBodyChild, MjIncludeBodyKind,
    };

    #[test]
    fn serialize_mjml() {
        let elt = MjIncludeBody::new(
            MjIncludeBodyAttributes::new("memory:include.mjml"),
            vec![MjIncludeBodyChild::MjButton(MjButton::default())],
        );
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-include","attributes":{"path":"memory:include.mjml"},"children":[{"type":"mj-button"}]}"#
        );
    }

    #[test]
    fn serialize_html() {
        let elt = MjIncludeBody::new(
            MjIncludeBodyAttributes::new("memory:include.html").with_kind(MjIncludeBodyKind::Html),
            vec![MjIncludeBodyChild::MjButton(MjButton::default())],
        );
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-include","attributes":{"path":"memory:include.html","type":"html"},"children":[{"type":"mj-button"}]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-include","attributes":{"path":"memory:include.mjml"},"children":[{"type":"mj-button"}]}"#;
        let res: MjIncludeBody = serde_json::from_str(json).unwrap();
        assert_eq!(res.0.attributes.path, "memory:include.mjml");
        assert!(!res.0.children.is_empty());
    }
}
