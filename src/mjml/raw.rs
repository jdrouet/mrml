use super::{Component, Error};
use crate::util::Context;
use roxmltree::Node;

#[derive(Clone, Debug)]
pub struct RawElement<'a, 'b> {
    context: Option<Context>,
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
    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx);
    }

    fn node(&self) -> Option<Node> {
        Some(self.node)
    }

    fn render(&self) -> Result<String, Error> {
        if self.node.is_text() {
            if let Some(txt) = self.node.text() {
                Ok(txt.into())
            } else {
                Ok("".into())
            }
        } else if self.node.is_comment() {
            match self.node.text() {
                Some(txt) => Ok(format!("<!--{}-->", txt)),
                None => Ok("".into()),
            }
        } else {
            Ok("raw".into())
        }
    }
}
