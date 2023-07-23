use super::MjStyleAttributes;

impl MjStyleAttributes {
    pub fn is_empty(&self) -> bool {
        self.inline.is_none()
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_style::MjStyle;

    #[test]
    fn serialize() {
        let elt = MjStyle::from("Hello World");
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-style","children":"Hello World"}"#
        );
    }

    #[test]
    fn deserialize() {
        let elt = MjStyle::from("Hello World");
        let json = serde_json::to_string(&elt).unwrap();
        let res: MjStyle = serde_json::from_str(&json).unwrap();
        assert_eq!(res.children, elt.children);
    }
}
