#[cfg(test)]
mod tests {
    use crate::mj_divider::MjDivider;

    #[test]
    fn serialize() {
        let mut elt = MjDivider::default();
        elt.attributes.insert("margin-bottom".into(), "20px".into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-divider","attributes":{"margin-bottom":"20px"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let elt = MjDivider::default();
        let json = serde_json::to_string(&elt).unwrap();
        let _res: MjDivider = serde_json::from_str(&json).unwrap();
    }
}
