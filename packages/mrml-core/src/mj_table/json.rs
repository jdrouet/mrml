#[cfg(test)]
mod tests {
    use crate::mj_table::MjTable;

    #[test]
    fn serialize() {
        let mut elt = MjTable::default();
        elt.attributes.insert("margin".into(), Some("42px".into()));
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-table","attributes":{"margin":"42px"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-table","attributes":{"margin-bottom":"20px"},"children":[{"type":"comment","children":"Hello World!"},"Hello World!"]}"#;
        let res: MjTable = serde_json::from_str(json).unwrap();
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.children.len(), 2);
    }
}
