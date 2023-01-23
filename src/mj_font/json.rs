use super::MjFontAttributes;

impl MjFontAttributes {
    pub fn is_empty(&self) -> bool {
        self.name.is_empty() && self.href.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_font::{MjFont, MjFontAttributes};

    #[test]
    fn serialize() {
        let elt = MjFont {
            attributes: MjFontAttributes {
                name: "Comic".to_string(),
                href: "somewhere".to_string(),
            },
        };
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-font","attributes":{"name":"Comic","href":"somewhere"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let elt = MjFont {
            attributes: MjFontAttributes {
                name: "Comic".to_string(),
                href: "somewhere".to_string(),
            },
        };
        let json = serde_json::to_string(&elt).unwrap();
        let res: MjFont = serde_json::from_str(&json).unwrap();
        assert_eq!(res.name(), elt.name());
        assert_eq!(res.href(), elt.href());
    }

    #[test]
    fn deserialize_missing_field() {
        let json = r#"{"type":"mj-font","attributes":{"name":"Comic"}}"#.to_string();
        assert!(serde_json::from_str::<MjFont>(&json).is_err());
    }
}
