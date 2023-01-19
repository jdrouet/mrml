use super::MJBreakpointAttributes;

impl MJBreakpointAttributes {
    pub fn is_empty(&self) -> bool {
        self.width.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_breakpoint::{MJBreakpoint, MJBreakpointAttributes};

    #[test]
    fn serialize() {
        let elt = MJBreakpoint {
            attributes: MJBreakpointAttributes {
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
        let elt = MJBreakpoint {
            attributes: MJBreakpointAttributes {
                width: "12px".to_string(),
            },
        };
        let json = serde_json::to_string(&elt).unwrap();
        let res: MJBreakpoint = serde_json::from_str(&json).unwrap();
        assert_eq!(res.value(), elt.value());
    }

    #[test]
    fn deserialize_missing_field() {
        let json = r#"{"type":"mj-breakpoint","attributes":{}}"#.to_string();
        assert!(serde_json::from_str::<MJBreakpoint>(&json).is_err());
    }
}
