use super::{Component, Error};
use crate::util::Properties;
use roxmltree::Node;

pub struct RawElement<'a, 'b> {
    context: Option<Properties>,
    node: Node<'a, 'b>,
}

impl RawElement<'_, '_> {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>) -> Result<RawElement<'a, 'b>, Error> {
        Ok(RawElement {
            context: None,
            node,
        })
    }
}

impl Component for RawElement<'_, '_> {
    fn default_attribute(_key: &str) -> Option<String> {
        None
    }

    fn set_context(&mut self, ctx: Properties) {
        self.context = Some(ctx);
    }

    fn node(&self) -> Option<Node> {
        Some(self.node)
    }

    fn render(&self) -> Result<String, Error> {
        if self.node.is_text() {
            if let Some(txt) = self.node.text() {
                Ok(txt.to_string())
            } else {
                Ok("".to_string())
            }
        } else {
            Ok("raw".into())
        }
    }
}
