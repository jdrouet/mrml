#[cfg(test)]
mod tests {
    use crate::{
        comment::Comment,
        mj_preview::{MjPreview, MjPreviewChild},
        prelude::OneOrMany,
        text::Text,
    };

    #[test]
    fn serialize() {
        let elt = MjPreview::from("Hello World");
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-preview","children":"Hello World"}"#
        );

        let mut elt = MjPreview::default();
        elt.children = OneOrMany::Many(vec![
            MjPreviewChild::Text(Text::from("Hello World")),
            MjPreviewChild::Comment(Comment::from("this is a comment")),
        ]);
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-preview","children":["Hello World",{"type":"comment","children":"this is a comment"}]}"#
        );
    }

    #[test]
    fn deserialize() {
        let elt = MjPreview::from("Hello World");
        let json = serde_json::to_string(&elt).unwrap();
        let res: MjPreview = serde_json::from_str(&json).unwrap();
        assert_eq!(res.children, elt.children);
    }

    #[test]
    fn deserialize_single_child() {
        // for backward compatibility
        let first: MjPreview =
            serde_json::from_str(r#"{"type":"mj-preview","children":"Hello World"}"#).unwrap();
        let second: MjPreview =
            serde_json::from_str(r#"{"type":"mj-preview","children":["Hello World"]}"#).unwrap();
        let first_children = Vec::from_iter(first.children.iter());
        let second_children = Vec::from_iter(second.children.iter());
        assert_eq!(first_children, second_children);
    }
}
