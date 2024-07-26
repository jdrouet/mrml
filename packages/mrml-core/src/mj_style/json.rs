use crate::prelude::json::ComponentAttributes;

use super::MjStyleAttributes;

impl ComponentAttributes for MjStyleAttributes {
    fn has_attributes(&self) -> bool {
        self.inline.is_some()
    }

    fn try_from_serde<Err: serde::de::Error>(this: Option<Self>) -> Result<Self, Err>
    where
        Self: Sized,
    {
        Ok(this.unwrap_or_default())
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
