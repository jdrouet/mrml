#[cfg(test)]
mod tests {
    use crate::fragment::Fragment;
    use crate::text::Text;

    #[test]
    fn serialize() {
        let mut elt = Fragment::<Text>::default();
        elt.children.push(Text::from("Hello"));
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"fragment","children":["Hello"]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"fragment","children":["Hello","World","!"]}"#;
        let res: Fragment<Text> = serde_json::from_str(json).unwrap();
        assert_eq!(res.children.len(), 3);
    }
}
