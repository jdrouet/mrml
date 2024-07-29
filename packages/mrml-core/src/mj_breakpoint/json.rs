use super::MjBreakpointAttributes;
use crate::prelude::json::ComponentAttributes;

impl ComponentAttributes for MjBreakpointAttributes {
    fn has_attributes(&self) -> bool {
        !self.width.is_empty()
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
    use crate::mj_breakpoint::{MjBreakpoint, MjBreakpointAttributes};

    #[test]
    fn serialize() {
        let elt = MjBreakpoint::new(
            MjBreakpointAttributes {
                width: "12px".to_string(),
            },
            (),
        );
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-breakpoint","attributes":{"width":"12px"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let elt = MjBreakpoint::new(
            MjBreakpointAttributes {
                width: "12px".to_string(),
            },
            (),
        );
        let json = serde_json::to_string(&elt).unwrap();
        let res: MjBreakpoint = serde_json::from_str(&json).unwrap();
        assert_eq!(res.value(), elt.value());
    }

    #[test]
    fn deserialize_missing_field() {
        let json = r#"{"type":"mj-breakpoint","attributes":{}}"#.to_string();
        assert!(serde_json::from_str::<MjBreakpoint>(&json).is_err());
    }
}
