#[cfg(test)]
mod tests {
    use crate::mj_body::{MjBody, MjBodyChild};
    use crate::text::Text;

    #[test]
    fn serialize() {
        let mut elt = MjBody::default();
        elt.attributes.insert("margin".into(), "42px".into());
        elt.children
            .push(MjBodyChild::Text(Text::from("Hello World!")));
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-body","attributes":{"margin":"42px"},"children":["Hello World!"]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-body","attributes":{"margin-bottom":"20px"},"children":[{"type":"comment","children":"Hello World!"},"Hello World!"]}"#;
        let res: MjBody = serde_json::from_str(json).unwrap();
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.children.len(), 2);
    }
}
