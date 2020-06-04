use crate::mjml::body::prelude::*;
use crate::mjml::body::BodyElement;
use crate::mjml::prelude::*;
use crate::mjml::{Component, Error};
use crate::util::prelude::PropertyMap;
use crate::util::{Attributes, Context, Header};
use crate::Options;
use crate::{close_tag, open_tag};
use roxmltree::Node;
use std::collections::HashMap;

fn empty_str() -> String {
    "".into()
}

fn get_node_text<'a, 'b>(node: Node<'a, 'b>) -> String {
    node.text()
        .and_then(|txt| Some(txt.to_string()))
        .or_else(|| Some(empty_str()))
        .unwrap()
}

#[derive(Clone, Debug)]
pub struct CommentElement {
    options: Options,
    content: String,
}

impl CommentElement {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>, opts: &Options) -> Result<CommentElement, Error> {
        Ok(Self {
            options: opts.clone(),
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

    fn render(&self, _header: &Header) -> Result<String, Error> {
        if !self.options.keep_comments || self.content.len() == 0 {
            return Ok(empty_str());
        }
        Ok(format!("<!--{}-->", self.content))
    }
}

#[derive(Clone, Debug)]
pub struct TextElement(String);

impl TextElement {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>, _opts: &Options) -> Result<TextElement, Error> {
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
    attributes: HashMap<String, String>,
    context: Option<Context>,
    children: Vec<BodyElement>,
    tag: String,
}

impl NodeElement {
    fn conditional_parse<'a, 'b>(
        node: Node<'a, 'b>,
        opts: &Options,
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
                    child, opts, true,
                )?))
            } else {
                children.push(BodyElement::parse(child, opts, None)?);
            }
        }
        Ok(NodeElement {
            attributes: get_node_attributes(&node),
            context: None,
            children,
            tag: node.tag_name().name().to_string(),
        })
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
        let mut attrs = Attributes::new();
        for (key, value) in self.attributes.iter() {
            attrs.set(key, value);
        }
        let mut res = vec![];
        res.push(open_tag!(self.tag, attrs.to_string()));
        for child in self.children.iter() {
            res.push(child.render(header)?);
        }
        res.push(close_tag!(self.tag));
        Ok(res.join(""))
    }
}

impl ComponentWithAttributes for NodeElement {
    fn source_attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.attributes)
    }
}

#[derive(Clone, Debug)]
pub enum RawElement {
    Comment(CommentElement),
    Node(NodeElement),
    Text(TextElement),
}

impl RawElement {
    fn parse_comment<'a, 'b>(node: Node<'a, 'b>, opts: &Options) -> Result<RawElement, Error> {
        CommentElement::parse(node, opts).and_then(|item| Ok(RawElement::Comment(item)))
    }

    fn parse_text<'a, 'b>(node: Node<'a, 'b>, opts: &Options) -> Result<RawElement, Error> {
        TextElement::parse(node, opts).and_then(|item| Ok(RawElement::Text(item)))
    }

    fn parse_node<'a, 'b>(
        node: Node<'a, 'b>,
        opts: &Options,
        only_raw: bool,
    ) -> Result<RawElement, Error> {
        NodeElement::conditional_parse(node, opts, only_raw)
            .and_then(|item| Ok(RawElement::Node(item)))
    }

    pub fn parse<'a, 'b>(node: Node<'a, 'b>, opts: &Options) -> Result<RawElement, Error> {
        RawElement::conditional_parse(node, opts, false)
    }

    pub fn conditional_parse<'a, 'b>(
        node: Node<'a, 'b>,
        opts: &Options,
        only_raw: bool,
    ) -> Result<RawElement, Error> {
        if node.is_comment() {
            RawElement::parse_comment(node, opts)
        } else if node.is_text() {
            RawElement::parse_text(node, opts)
        } else {
            RawElement::parse_node(node, opts, only_raw)
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
    fn source_attributes(&self) -> Option<&HashMap<String, String>> {
        match self {
            RawElement::Node(node) => node.source_attributes(),
            _ => None,
        }
    }
}

impl BodyComponent for RawElement {}
