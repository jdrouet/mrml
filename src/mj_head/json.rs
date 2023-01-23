#[cfg(test)]
mod tests {
    use crate::mj_head::MjHead;

    #[test]
    fn serialize() {
        let elt = MjHead::default();
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-head"}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-head","children":[{"type":"mj-font","attributes":{"name":"Comic","href":"http://jolimail.io"}},{"type":"mj-breakpoint","attributes":{"width":"12px"}}]}"#;
        let res: MjHead = serde_json::from_str(json).unwrap();
        assert_eq!(res.children.len(), 2);
        let next = serde_json::to_string(&res).unwrap();
        assert_eq!(next, json);
    }
}
