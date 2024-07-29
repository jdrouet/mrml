use crate::prelude::json::JsonAttributes;

impl JsonAttributes for super::MjAttributesClassAttributes {
    fn has_attributes(&self) -> bool {
        true
    }

    fn try_from_serde<Err: serde::de::Error>(this: Option<Self>) -> Result<Self, Err>
    where
        Self: Sized,
    {
        this.ok_or_else(|| serde::de::Error::missing_field("attributes"))
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_attributes_class::{MjAttributesClass, MjAttributesClassAttributes};

    #[test]
    fn serialize() {
        let mut elt = MjAttributesClass::new(
            MjAttributesClassAttributes {
                name: "classname".into(),
                others: Default::default(),
            },
            (),
        );
        elt.attributes
            .others
            .insert("margin-bottom".into(), "20px".into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-class","attributes":{"name":"classname","margin-bottom":"20px"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let elt = MjAttributesClass::new(MjAttributesClassAttributes::new("classname".into()), ());
        let json = serde_json::to_string(&elt).unwrap();
        let _res: MjAttributesClass = serde_json::from_str(&json).unwrap();
    }
}
