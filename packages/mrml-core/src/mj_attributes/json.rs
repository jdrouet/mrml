#[cfg(test)]
mod tests {
    use crate::mj_attributes::MjAttributes;
    use crate::mj_attributes_all::MjAttributesAll;
    use crate::mj_attributes_class::MjAttributesClass;

    #[test]
    fn serialize() {
        let mut elt = MjAttributes::default();
        elt.children.push(MjAttributesAll::default().into());
        elt.children
            .push(MjAttributesClass::new("name".into()).into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-attributes","children":[{"type":"mj-all"},{"type":"mj-class","name":"name"}]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-attributes","children":[{"type":"mj-all"},{"type":"mj-class","name":"name"}]}"#;
        let res: MjAttributes = serde_json::from_str(json).unwrap();
        assert_eq!(res.children.len(), 2);
        let next = serde_json::to_string(&res).unwrap();
        assert_eq!(next, json);
    }
}
