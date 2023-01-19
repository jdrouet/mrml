#[cfg(test)]
mod tests {
    use crate::mj_image::MJImage;
    use crate::prelude::hash::Map;

    #[test]
    fn serialize() {
        let mut attrs = Map::new();
        attrs.insert("href".to_string(), "https://jolimail.io".to_string());
        let elt = MJImage { attributes: attrs };
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-image","attributes":{"href":"https://jolimail.io"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-image","attributes":{"href":"https://jolimail.io"}}"#;
        let res: MJImage = serde_json::from_str(json).unwrap();
        assert_eq!(res.attributes.get("href").unwrap(), "https://jolimail.io");
    }
}
