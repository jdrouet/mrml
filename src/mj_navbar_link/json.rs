#[cfg(test)]
mod tests {
    use crate::mj_navbar_link::MjNavbarLink;

    #[test]
    fn serialize() {
        let mut elt = MjNavbarLink::default();
        elt.attributes
            .insert("href".into(), "https://jolimail.io".into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-navbar-link","attributes":{"href":"https://jolimail.io"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-navbar-link","attributes":{"margin-bottom":"20px"},"children":["Hello",{"type":"comment","children":"World"}]}"#;
        let res: MjNavbarLink = serde_json::from_str(json).unwrap();
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.children.len(), 2);
    }
}
