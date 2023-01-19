#[cfg(test)]
mod tests {
    use crate::mj_spacer::MJSpacer;

    #[test]
    fn serialize() {
        let mut elt = MJSpacer::default();
        elt.attributes.insert("margin-bottom".into(), "20px".into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-spacer","attributes":{"margin-bottom":"20px"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let elt = MJSpacer::default();
        let json = serde_json::to_string(&elt).unwrap();
        let _res: MJSpacer = serde_json::from_str(&json).unwrap();
    }
}
