use std::fmt;

use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::MjmlChildren;
use crate::mj_body::MjBody;
use crate::mj_head::MjHead;
use crate::prelude::json::{JsonAttributes, JsonChildren};

impl JsonAttributes for super::MjmlAttributes {
    fn has_attributes(&self) -> bool {
        self.owa.is_some() || self.lang.is_some() || self.dir.is_some()
    }

    fn try_from_serde<Err: serde::de::Error>(this: Option<Self>) -> Result<Self, Err>
    where
        Self: Sized,
    {
        Ok(this.unwrap_or_default())
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum MjmlChild {
    MjHead(MjHead),
    MjBody(MjBody),
}

impl JsonChildren for MjmlChildren {
    fn has_children(&self) -> bool {
        self.head.is_some() || self.body.is_some()
    }

    fn try_from_serde<Err: serde::de::Error>(this: Option<Self>) -> Result<Self, Err>
    where
        Self: Sized,
    {
        Ok(this.unwrap_or_default())
    }
}

impl MjmlChildren {
    pub fn is_empty(&self) -> bool {
        self.head.is_none() && self.body.is_none()
    }
}

impl Serialize for MjmlChildren {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_seq(Some(2))?;
        if let Some(ref head) = self.head {
            map.serialize_element(head)?;
        }
        if let Some(ref body) = self.body {
            map.serialize_element(body)?;
        }
        map.end()
    }
}

#[derive(Default)]
struct MjmlChildrenVisitor;

impl<'de> Visitor<'de> for MjmlChildrenVisitor {
    type Value = MjmlChildren;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence with title and text elements")
    }

    fn visit_seq<S>(self, mut access: S) -> Result<Self::Value, S::Error>
    where
        S: SeqAccess<'de>,
    {
        let mut result = MjmlChildren::default();
        while let Some(value) = access.next_element::<MjmlChild>()? {
            match value {
                MjmlChild::MjHead(head) => result.head = Some(head),
                MjmlChild::MjBody(body) => result.body = Some(body),
            };
        }
        Ok(result)
    }
}

impl<'de> Deserialize<'de> for MjmlChildren {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(MjmlChildrenVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::mjml::Mjml;

    #[test]
    fn serialize() {
        let elt = Mjml::default();
        assert_eq!(serde_json::to_string(&elt).unwrap(), r#"{"type":"mjml"}"#);
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mjml","attributes":{"lang":"fr"},"children":[{"type":"mj-head"},{"type":"mj-body","children":["Hello World!"]}]}"#;
        let res: Mjml = serde_json::from_str(json).unwrap();
        assert!(res.children.head.is_some());
        assert!(res.children.body.is_some());
        let next = serde_json::to_string(&res).unwrap();
        assert_eq!(next, json);
    }
}
