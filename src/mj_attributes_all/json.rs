#[cfg(test)]
mod tests {
    use crate::mj_attributes_all::MjAttributesAll;

    #[test]
    fn serialize() {
        let mut elt = MjAttributesAll::default();
        elt.attributes.insert("margin-bottom".into(), "20px".into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-all","attributes":{"margin-bottom":"20px"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let elt = MjAttributesAll::default();
        let json = serde_json::to_string(&elt).unwrap();
        let _res: MjAttributesAll = serde_json::from_str(&json).unwrap();
    }
}
