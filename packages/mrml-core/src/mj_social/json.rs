#[cfg(test)]
mod tests {
    use crate::mj_social::MjSocial;

    #[test]
    fn serialize() {
        let mut elt = MjSocial::default();
        elt.attributes.insert("margin".into(), "42px".into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-social","attributes":{"margin":"42px"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-social","attributes":{"margin":"0px"},"children":[{"type":"mj-social-element","attributes":{"name":"twitter"}},{"type":"comment","children":"World"}]}"#;
        let res: MjSocial = serde_json::from_str(json).unwrap();
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.children.len(), 2);
    }
}
