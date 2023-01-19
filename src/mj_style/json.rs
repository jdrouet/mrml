use super::MjStyleAttributes;

impl MjStyleAttributes {
    pub fn is_empty(&self) -> bool {
        self.inline.is_none()
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_style::MJStyle;

    #[test]
    fn serialize() {
        let elt = MJStyle::from("Hello World");
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-style","children":"Hello World"}"#
        );
    }

    #[test]
    fn deserialize() {
        let elt = MJStyle::from("Hello World");
        let json = serde_json::to_string(&elt).unwrap();
        let res: MJStyle = serde_json::from_str(&json).unwrap();
        assert_eq!(res.children, elt.children);
    }
}
