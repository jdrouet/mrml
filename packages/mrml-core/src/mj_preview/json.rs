#[cfg(test)]
mod tests {
    use crate::mj_preview::MjPreview;

    #[test]
    fn serialize() {
        let elt = MjPreview::from("Hello World");
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-preview","children":"Hello World"}"#
        );
    }

    #[test]
    fn deserialize() {
        let elt = MjPreview::from("Hello World");
        let json = serde_json::to_string(&elt).unwrap();
        let res: MjPreview = serde_json::from_str(&json).unwrap();
        assert_eq!(res.children, elt.children);
    }
}
