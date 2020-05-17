use super::{Component, Element, Error};
use crate::util::prelude::PropertyMap;
use crate::util::{Attributes, Context};
use crate::{open_tag, close_tag};
use roxmltree::Node;

#[derive(Clone, Debug)]
pub struct RawElement<'a, 'b> {
    context: Option<Context>,
    node: Node<'a, 'b>,
    children: Vec<Element<'a, 'b>>,
}

fn empty_str() -> String {
    "".into()
}

impl RawElement<'_, '_> {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>) -> Result<RawElement<'a, 'b>, Error> {
        let mut children = vec![];
        for child in node.children() {
            children.push(Element::parse(child)?);
        }
        Ok(RawElement {
            context: None,
            node,
            children,
        })
    }

    fn render_comment(&self) -> String {
        let keep_comments = self
            .context()
            .and_then(|ctx| Some(ctx.options().keep_comments))
            .or_else(|| Some(true))
            .unwrap();
        if !keep_comments {
            return empty_str();
        }
        match self.node.text() {
            Some(txt) => format!("<!--{}-->", txt),
            None => empty_str(),
        }
    }

    fn render_component(&self) -> Result<String, Error> {
        let mut attrs = Attributes::new();
        for attr in self.node.attributes().iter() {
            attrs.set(attr.name(), attr.value());
        }
        let mut res = vec![];
        res.push(open_tag!(self.node.tag_name().name(), attrs.to_string()));
        for child in self.children.iter() {
            res.push(child.render()?);
        }
        res.push(close_tag!(self.node.tag_name().name()));
        Ok(res.join(""))
    }

    fn render_text(&self) -> String {
        match self.node.text() {
            Some(txt) => txt.trim().into(),
            None => empty_str(),
        }
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

    fn is_raw(&self) -> bool {
        true
    }

    fn render(&self) -> Result<String, Error> {
        if self.node.is_text() {
            Ok(self.render_text())
        } else if self.node.is_comment() {
            Ok(self.render_comment())
        } else {
            self.render_component()
        }
    }
}
