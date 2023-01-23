#[cfg(test)]
mod tests {
    use crate::mj_title::MjTitle;

    #[test]
    fn serialize() {
        let elt = MjTitle::from("Hello World");
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-title","children":"Hello World"}"#
        );
    }

    #[test]
    fn deserialize() {
        let elt = MjTitle::from("Hello World");
        let json = serde_json::to_string(&elt).unwrap();
        let res: MjTitle = serde_json::from_str(&json).unwrap();
        assert_eq!(res.children, elt.children);
    }
}
