use std::fmt;

use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::comment::Comment;
use crate::mj_accordion_text::MjAccordionText;
use crate::mj_accordion_title::MjAccordionTitle;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
enum MjAccordionElementChild {
    Comment(Comment),
    MjAccordionText(MjAccordionText),
    MjAccordionTitle(MjAccordionTitle),
}

use super::MjAccordionElementChildren;

impl MjAccordionElementChildren {
    pub fn is_empty(&self) -> bool {
        self.title.is_none() && self.text.is_none()
    }
}

impl Serialize for MjAccordionElementChildren {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_seq(Some(2))?;
        if let Some(ref title) = self.title {
            map.serialize_element(title)?;
        }
        if let Some(ref text) = self.text {
            map.serialize_element(text)?;
        }
        map.end()
    }
}

#[derive(Default)]
struct MjAccordionElementChildrenVisitor;

impl<'de> Visitor<'de> for MjAccordionElementChildrenVisitor {
    type Value = MjAccordionElementChildren;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence with title and text elements")
    }

    fn visit_seq<S>(self, mut access: S) -> Result<Self::Value, S::Error>
    where
        S: SeqAccess<'de>,
    {
        let mut result = MjAccordionElementChildren::default();
        while let Some(value) = access.next_element::<MjAccordionElementChild>()? {
            match value {
                MjAccordionElementChild::MjAccordionTitle(title) => result.title = Some(title),
                MjAccordionElementChild::MjAccordionText(text) => result.text = Some(text),
                _ => (),
            };
        }
        Ok(result)
    }
}

impl<'de> Deserialize<'de> for MjAccordionElementChildren {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(MjAccordionElementChildrenVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_accordion_element::MjAccordionElement;
    use crate::mj_accordion_title::MjAccordionTitle;
    use crate::text::Text;

    #[test]
    fn serialize() {
        let mut elt = MjAccordionElement::default();
        elt.attributes
            .insert("margin".to_string(), "12px".to_string());
        elt.children.title = Some(MjAccordionTitle {
            attributes: Default::default(),
            children: vec![Text::from("Hello".to_string())],
        });
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-accordion-element","attributes":{"margin":"12px"},"children":[{"type":"mj-accordion-title","children":["Hello"]}]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-accordion-element","attributes":{"margin":"12px"},"children":[{"type":"mj-accordion-title"},{"type":"mj-accordion-text"}]}"#;
        let res: MjAccordionElement = serde_json::from_str(json).unwrap();
        assert!(res.children.text.is_some());
        assert!(res.children.title.is_some());
        let next = serde_json::to_string(&res).unwrap();
        assert_eq!(next, json);
    }
}
