#[cfg(test)]
mod tests {
    use crate::mj_attributes::{MjAttributes, MjAttributesChild};
    use crate::mj_attributes_all::MjAttributesAll;
    use crate::mj_attributes_class::{MjAttributesClass, MjAttributesClassAttributes};

    #[test]
    fn serialize() {
        let mut elt = MjAttributes::default();
        elt.children.push(MjAttributesChild::MjAttributesAll(
            MjAttributesAll::default(),
        ));
        elt.children.push(MjAttributesChild::MjAttributesClass(
            MjAttributesClass::new(
                MjAttributesClassAttributes {
                    name: "name".into(),
                    others: Default::default(),
                },
                (),
            ),
        ));
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-attributes","children":[{"type":"mj-all"},{"type":"mj-class","attributes":{"name":"name"}}]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-attributes","children":[{"type":"mj-all"},{"type":"mj-class","attributes":{"name":"name"}}]}"#;
        let res: MjAttributes = serde_json::from_str(json).unwrap();
        assert_eq!(res.children.len(), 2);
        let next = serde_json::to_string(&res).unwrap();
        assert_eq!(next, json);
    }
}
