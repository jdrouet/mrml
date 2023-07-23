#[cfg(test)]
mod tests {
    use crate::comment::Comment;

    #[test]
    fn serialize() {
        let elt = Comment::from("Hello World");
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"comment","children":"Hello World"}"#
        );
    }

    #[test]
    fn deserialize() {
        let elt = Comment::from("Hello World");
        let json = serde_json::to_string(&elt).unwrap();
        let res: Comment = serde_json::from_str(&json).unwrap();
        assert_eq!(res.children, elt.children);
        // invalid attributes
        assert!(serde_json::from_str::<Comment>(r#"{"type":"comment","toto":"tata"}"#).is_err())
    }
}
