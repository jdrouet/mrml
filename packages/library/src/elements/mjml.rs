use super::body::mj_body::MJBody;
use super::head::mj_head::MJHead;
use super::prelude::*;
use super::Error;
use crate::parser::{Element, Node};
use crate::util::context::Context;
use crate::Options;
use log::debug;

#[derive(Clone, Debug)]
pub struct MJMLElement<'a> {
    context: Option<Context>,
    head: MJHead<'a>,
    body: MJBody,
}

impl<'a> MJMLElement<'a> {
    pub fn parse(node: &Node<'a>, opts: Options) -> Result<MJMLElement<'a>, Error> {
        let mut head: Option<&Node<'a>> = None;
        let mut body: Option<&Node<'a>> = None;
        for item in node.children.iter() {
            match item {
                Element::Node(node) => match node.name.as_str() {
                    "mj-head" => head = Some(node),
                    "mj-body" => body = Some(node),
                    name => return Err(Error::UnexpectedElement(name.into())),
                },
                // TODO handle comments in <mjml>
                Element::Comment(_) => (),
                Element::Text(_) => return Err(Error::UnexpectedText),
            };
        }
        let mut head = match head {
            Some(node) => MJHead::parse(node, opts)?,
            None => MJHead::empty(opts),
        };
        let mut body = match body {
            Some(node) => MJBody::parse(node, head.get_header())?,
            None => MJBody::empty(),
        };
        body.set_context(Context::default());
        body.update_header(head.get_mut_header());
        let element = MJMLElement {
            context: None,
            head,
            body,
        };
        Ok(element)
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
