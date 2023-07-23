#[cfg(test)]
mod tests {
    use crate::node::Node;
    use crate::text::Text;

    #[test]
    fn serialize() {
        let mut elt = Node::<Text>::from("span");
        elt.attributes.insert("margin-bottom".into(), "20px".into());
        elt.children.push(Text::from("Hello"));
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"span","attributes":{"margin-bottom":"20px"},"children":["Hello"]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"span","attributes":{"margin-bottom":"20px"},"children":["Hello","World","!"]}"#;
        let res: Node<Text> = serde_json::from_str(json).unwrap();
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.children.len(), 3);
    }
}
