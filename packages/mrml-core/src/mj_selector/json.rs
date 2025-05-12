use crate::prelude::json::JsonAttributes;

impl JsonAttributes for super::MjSelectorAttributes {
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
    use crate::{
        mj_html_attribute::MjHtmlAttribute, mj_selector::MjSelector,
        mj_selector::MjSelectorAttributes,
    };

    #[test]
    fn serialize() {
        let elt = MjSelector::new(
            MjSelectorAttributes {
                path: ".test".into(),
            },
            Vec::<MjHtmlAttribute>::new(),
        );

        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-selector","attributes":{"path":".test"}}"#,
        );
    }
}
