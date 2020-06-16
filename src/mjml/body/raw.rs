use crate::mjml::body::prelude::*;
use crate::mjml::body::BodyElement;
use crate::mjml::prelude::*;
use crate::mjml::{Component, Error};
use crate::util::attributes::*;
use crate::util::{Context, Header, Tag};
use roxmltree::Node;
use std::collections::HashMap;

fn empty_str() -> String {
    "".into()
}

fn get_node_text<'a, 'b>(node: &Node<'a, 'b>) -> String {
    node.text()
        .and_then(|txt| Some(txt.to_string()))
        .or_else(|| Some(empty_str()))
        .unwrap()
}

#[derive(Clone, Debug)]
pub struct CommentElement {
    content: String,
}

impl CommentElement {
    pub fn parse<'a, 'b>(node: &Node<'a, 'b>, _header: &Header) -> Result<CommentElement, Error> {
        Ok(Self {
            content: get_node_text(node),
        })
    }
}

impl Component for CommentElement {
    fn context(&self) -> Option<&Context> {
        None
    }

    fn set_context(&mut self, _ctx: Context) {
        // noop
    }

    fn render(&self, header: &Header) -> Result<String, Error> {
        if !header.keep_comments() || self.content.len() == 0 {
            return Ok(empty_str());
        }
        Ok(format!("<!--{}-->", self.content))
    }
}

#[derive(Clone, Debug)]
pub struct TextElement(String);

impl TextElement {
    pub fn parse<'a, 'b>(node: &Node<'a, 'b>, _header: &Header) -> Result<TextElement, Error> {
        Ok(Self(get_node_text(node)))
    }
}

impl Component for TextElement {
    fn context(&self) -> Option<&Context> {
        None
    }

    fn set_context(&mut self, _ctx: Context) {
        // noop
    }

    fn render(&self, _header: &Header) -> Result<String, Error> {
        Ok(self.0.clone())
    }
}

#[derive(Clone, Debug)]
pub struct NodeElement {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<BodyElement>,
    tag: String,
}

impl NodeElement {
    fn conditional_parse<'a, 'b>(
        node: &Node<'a, 'b>,
        header: &Header,
        only_raw: bool,
    ) -> Result<NodeElement, Error> {
        let tag = node.tag_name().name();
        if only_raw && tag.starts_with("mj-") {
            return Err(Error::ParseError(format!("'{}' is not allowed", tag)));
        }
        let mut children = vec![];
        for child in node.children() {
            if only_raw {
                children.push(BodyElement::Raw(RawElement::conditional_parse(
                    &child, header, true,
                )?))
            } else {
                children.push(BodyElement::parse(&child, header, None)?);
            }
        }
        Ok(NodeElement {
            attributes: Attributes::from(node),
            context: None,
            children,
            tag: node.tag_name().name().to_string(),
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
    Comment(CommentElement),
    Node(NodeElement),
    Text(TextElement),
}

impl RawElement {
    fn parse_comment<'a, 'b>(node: &Node<'a, 'b>, header: &Header) -> Result<RawElement, Error> {
        CommentElement::parse(node, header).and_then(|item| Ok(RawElement::Comment(item)))
    }

    fn parse_text<'a, 'b>(node: &Node<'a, 'b>, header: &Header) -> Result<RawElement, Error> {
        TextElement::parse(node, header).and_then(|item| Ok(RawElement::Text(item)))
    }

    fn parse_node<'a, 'b>(
        node: &Node<'a, 'b>,
        header: &Header,
        only_raw: bool,
    ) -> Result<RawElement, Error> {
        NodeElement::conditional_parse(node, header, only_raw)
            .and_then(|item| Ok(RawElement::Node(item)))
    }

    pub fn parse<'a, 'b>(node: &Node<'a, 'b>, header: &Header) -> Result<RawElement, Error> {
        RawElement::conditional_parse(node, header, false)
    }

    pub fn conditional_parse<'a, 'b>(
        node: &Node<'a, 'b>,
        header: &Header,
        only_raw: bool,
    ) -> Result<RawElement, Error> {
        if node.is_comment() {
            RawElement::parse_comment(node, header)
        } else if node.is_text() {
            RawElement::parse_text(node, header)
        } else {
            RawElement::parse_node(node, header, only_raw)
        }
    }
}

impl Component for RawElement {
    fn context(&self) -> Option<&Context> {
        match self {
            RawElement::Comment(comment) => comment.context(),
            RawElement::Node(node) => node.context(),
            RawElement::Text(_) => None,
        }
    }

    fn set_context(&mut self, ctx: Context) {
        match self {
            RawElement::Comment(comment) => comment.set_context(ctx),
            RawElement::Node(node) => node.set_context(ctx),
            RawElement::Text(_) => (),
        };
    }

    fn render(&self, header: &Header) -> Result<String, Error> {
        match self {
            RawElement::Comment(comment) => comment.render(header),
            RawElement::Node(node) => node.render(header),
            RawElement::Text(element) => element.render(header),
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
