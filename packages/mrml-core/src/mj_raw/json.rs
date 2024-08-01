#[cfg(test)]
mod tests {
    use crate::mj_raw::{MjRaw, MjRawChild};
    use crate::text::Text;

    #[test]
    fn serialize() {
        let mut elt = MjRaw::default();
        elt.children.push(MjRawChild::Text(Text::from("Hello")));
        elt.children.push(MjRawChild::Text(Text::from("World")));
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-raw","children":["Hello","World"]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-raw","children":["Hello","World"]}"#;
        let res: MjRaw = serde_json::from_str(json).unwrap();
        assert_eq!(res.children.len(), 2);
        let next = serde_json::to_string(&res).unwrap();
        assert_eq!(next, json);
    }
}
