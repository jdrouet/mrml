use super::body::mj_body::{MJBody, NAME as MJ_BODY};
use super::head::mj_head::{MJHead, NAME as MJ_HEAD};
use super::prelude::*;
use super::Error;
use crate::parser::{Element, MJMLParser, Node};
use crate::util::context::Context;
use crate::Options;
use log::debug;

#[derive(Clone, Debug)]
pub struct MJMLElement {
    context: Option<Context>,
    head: MJHead,
    body: MJBody,
}

struct MJMLElementParser {
    options: Options,
    context: Option<Context>,
    head: Option<MJHead>,
    body: Option<MJBody>,
}

impl MJMLElementParser {
    pub fn new(options: Options) -> Self {
        Self {
            options,
            context: None,
            head: None,
            body: None,
        }
    }
}

impl MJMLParser for MJMLElementParser {
    type Output = MJMLElement;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJMLElement {
            context: self.context,
            head: self.head.unwrap(),
            body: self.body.unwrap(),
        })
    }

    fn parse<'a>(mut self, node: &Node<'a>) -> Result<Self, Error> {
        let mut head: Option<&Node<'a>> = None;
        let mut body: Option<&Node<'a>> = None;
        for item in node.children.iter() {
            match item {
                Element::Node(node) => match node.name.as_str() {
                    MJ_HEAD => head = Some(node),
                    MJ_BODY => body = Some(node),
                    name => return Err(Error::UnexpectedElement(name.into())),
                },
                // TODO handle comments in <mjml>
                Element::Comment(_) => (),
                Element::Text(_) => return Err(Error::UnexpectedText),
            };
        }
        let mut head = match head {
            Some(node) => MJHead::parse(node, self.options.clone())?,
            None => MJHead::empty(self.options.clone()),
        };
        let mut body = match body {
            Some(node) => MJBody::parse(node, head.get_header())?,
            None => MJBody::empty(),
        };
        body.set_context(Context::default());
        body.update_header(head.get_mut_header());
        self.head = Some(head);
        self.body = Some(body);
        Ok(self)
    }
}

impl<'a> MJMLElement {
    pub fn parse(node: &Node<'a>, opts: Options) -> Result<MJMLElement, Error> {
        MJMLElementParser::new(opts).parse(node)?.build()
    }

    pub fn get_title(&self) -> String {
        debug!("get_title");
        self.head.get_title()
    }

    pub fn get_preview(&self) -> String {
        debug!("get_preview");
        self.head.get_preview()
    }

    pub fn get_html(&self) -> Result<String, Error> {
        debug!("get_html");
        let header = self.head.get_header();
        Ok(String::from("<!doctype html>")
            + "<html xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:v=\"urn:schemas-microsoft-com:vml\" xmlns:o=\"urn:schemas-microsoft-com:office:office\">"
            + self.head.render(&header)?.as_str()
            + self.body.render(&header)?.as_str()
            + "</html>")
    }
}
