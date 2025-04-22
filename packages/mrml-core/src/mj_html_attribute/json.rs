use crate::prelude::json::JsonAttributes;

impl JsonAttributes for super::MjHtmlAttributeAttributes {
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
    use crate::mj_html_attribute::{MjHtmlAttribute, MjHtmlAttributeAttributes};

    #[test]
    fn serialize() {
        let elt = MjHtmlAttribute::new(
            MjHtmlAttributeAttributes {
                name: ".classname".into(),
            },
            "42".into(),
        );
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-html-attribute","attributes":{"name":".classname"},"children":"42"}"#
        );
    }
}
