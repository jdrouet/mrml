mod parser;

use crate::elements::body::prelude::*;
use crate::elements::body::BodyElement;
use crate::elements::{Component, Error};
use crate::util::attributes::Attributes;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;
use crate::util::tag::Tag;

#[derive(Clone, Debug)]
pub struct NodeElement {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<BodyElement>,
    tag: String,
}

impl NodeElement {
    fn closed_element(&self) -> bool {
        self.children.is_empty() && ["img"].contains(&self.tag.as_str())
    }
}

impl Component for NodeElement {
    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx);
    }

    fn render(&self, header: &Header) -> Result<String, Error> {
        let tag = Tag::new(self.tag.as_str()).insert_attributes(self.attributes.inner());
        if self.closed_element() {
            Ok(tag.closed())
        } else {
            let mut res = vec![];
            for child in self.children.iter() {
                res.push(child.render(header)?);
            }
            Ok(tag.render(res.join("")))
        }
    }
}

impl BodyComponent for NodeElement {
    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }
    fn get_children(&self) -> &Vec<BodyElement> {
        &self.children
    }

    fn get_current_width(&self) -> Option<Size> {
        None
    }
}

#[derive(Clone, Debug)]
pub enum RawElement {
    Comment(String),
    Node(NodeElement),
    Text(String),
}

impl RawElement {
    pub fn as_node(&self) -> Option<&NodeElement> {
        match self {
            RawElement::Node(node) => Some(node),
            _ => None,
        }
    }
}

impl Component for RawElement {
    fn context(&self) -> Option<&Context> {
        self.as_node().and_then(|node| node.context())
    }

    fn set_context(&mut self, ctx: Context) {
        if let RawElement::Node(node) = self {
            node.set_context(ctx);
        }
    }

    fn render(&self, header: &Header) -> Result<String, Error> {
        match self {
            RawElement::Comment(value) => {
                if header.keep_comments {
                    Ok(format!("<!-- {} -->", value))
                } else {
                    Ok(String::new())
                }
            }
            RawElement::Node(node) => node.render(header),
            RawElement::Text(value) => Ok(value.clone()),
        }
    }
}

impl BodyComponent for RawElement {
    fn get_children(&self) -> &Vec<BodyElement> {
        self.as_node()
            .map(|node| node.get_children())
            .unwrap_or(&EMPTY_CHILDREN)
    }

    fn get_current_width(&self) -> Option<Size> {
        self.as_node().and_then(|node| node.get_current_width())
    }

    fn attributes(&self) -> Option<&Attributes> {
        self.as_node().and_then(|node| node.attributes())
    }
}
