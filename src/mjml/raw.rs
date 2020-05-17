use super::{Component, Element, Error};
use super::prelude::get_node_attributes;
use crate::util::prelude::PropertyMap;
use crate::util::{Attributes, Context};
use crate::{close_tag, open_tag};
use roxmltree::Node;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct RawElement<'a, 'b> {
    attributes: HashMap<String, String>,
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
            attributes: get_node_attributes(&node),
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
        for (key, value) in self.attributes.iter() {
            attrs.set(key, value);
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

    fn source_attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.attributes)
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
