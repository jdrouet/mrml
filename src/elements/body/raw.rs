use crate::elements::body::prelude::*;
use crate::elements::body::BodyElement;
use crate::elements::prelude::*;
use crate::elements::{Component, Error};
use crate::parser::{Element, Node};
use crate::util::attributes::*;
use crate::util::{Context, Header, Tag};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct NodeElement {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<BodyElement>,
    tag: String,
}

impl NodeElement {
    fn conditional_parse<'a>(
        node: &Node<'a>,
        header: &Header,
        only_raw: bool,
    ) -> Result<NodeElement, Error> {
        let tag = node.name.as_str();
        if only_raw && tag.starts_with("mj-") {
            return Err(Error::ParseError(format!("'{}' is not allowed", tag)));
        }
        let mut children = vec![];
        for child in node.children.iter() {
            if only_raw {
                children.push(BodyElement::Raw(RawElement::conditional_parse(
                    child, header, true,
                )?))
            } else {
                children.push(BodyElement::parse(&child, header, None)?);
            }
        }
        Ok(NodeElement {
            attributes: Attributes::from(node),
            context: None,
            children,
            tag: node.name.as_str().to_string(),
        })
    }

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

impl ComponentWithAttributes for NodeElement {
    fn attributes(&self) -> Option<&HashMap<String, String>> {
        Some(self.attributes.inner())
    }
}

#[derive(Clone, Debug)]
pub enum RawElement {
    Comment(String),
    Node(NodeElement),
    Text(String),
}

impl RawElement {
    pub fn parse<'a>(element: &Element<'a>, header: &Header) -> Result<RawElement, Error> {
        RawElement::conditional_parse(element, header, false)
    }

    pub fn conditional_parse<'a>(
        element: &Element<'a>,
        header: &Header,
        only_raw: bool,
    ) -> Result<RawElement, Error> {
        match element {
            Element::Text(value) => Ok(RawElement::Text(value.as_str().into())),
            Element::Comment(value) => Ok(RawElement::Comment(value.as_str().into())),
            Element::Node(node) => Ok(RawElement::Node(NodeElement::conditional_parse(
                node, header, only_raw,
            )?)),
        }
    }
}

impl Component for RawElement {
    fn context(&self) -> Option<&Context> {
        match self {
            RawElement::Node(node) => node.context(),
            _ => None,
        }
    }

    fn set_context(&mut self, ctx: Context) {
        match self {
            RawElement::Node(node) => node.set_context(ctx),
            _ => (),
        };
    }

    fn render(&self, header: &Header) -> Result<String, Error> {
        match self {
            RawElement::Comment(value) => {
                if header.keep_comments() {
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

impl ComponentWithAttributes for RawElement {
    fn attributes(&self) -> Option<&HashMap<String, String>> {
        match self {
            RawElement::Node(node) => node.attributes(),
            _ => None,
        }
    }
}

impl BodyComponent for RawElement {}
