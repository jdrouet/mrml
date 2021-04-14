use super::{MJBreakpoint, MJBreakpointAttributes, NAME};
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

const FIELDS: [&str; 2] = ["type", "attributes"];

impl Serialize for MJBreakpoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("type", NAME)?;
        map.serialize_entry("attributes", &self.attributes)?;
        map.end()
    }
}

#[derive(Default)]
struct MJBreakpointVisitor;

impl<'de> Visitor<'de> for MJBreakpointVisitor {
    type Value = MJBreakpoint;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an map with properties type and attributes")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut elt = MJBreakpoint::default();
        while let Some(key) = access.next_key::<String>()? {
            if key == "type" {
                if access.next_value::<String>()? != NAME {
                    return Err(M::Error::custom(format!("expected type to equal {}", NAME)));
                }
            } else if key == "attributes" {
                elt.attributes = access.next_value::<MJBreakpointAttributes>()?;
            } else {
                return Err(M::Error::unknown_field(&key, &FIELDS));
            }
        }
        Ok(elt)
    }
}

impl<'de> Deserialize<'de> for MJBreakpoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MJBreakpointVisitor::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_breakpoint::{MJBreakpoint, MJBreakpointAttributes};

    #[test]
    fn serialize() {
        let elt = MJBreakpoint {
            attributes: MJBreakpointAttributes {
                value: "12px".to_string(),
            },
        };
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-breakpoint","attributes":{"value":"12px"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let elt = MJBreakpoint {
            attributes: MJBreakpointAttributes {
                value: "12px".to_string(),
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
