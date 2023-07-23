use super::MjBreakpointAttributes;

impl MjBreakpointAttributes {
    pub fn is_empty(&self) -> bool {
        self.width.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_breakpoint::{MjBreakpoint, MjBreakpointAttributes};

    #[test]
    fn serialize() {
        let elt = MjBreakpoint {
            attributes: MjBreakpointAttributes {
                width: "12px".to_string(),
            },
        };
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-breakpoint","attributes":{"width":"12px"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let elt = MjBreakpoint {
            attributes: MjBreakpointAttributes {
                width: "12px".to_string(),
            },
        };
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
