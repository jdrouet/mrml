use super::body::mj_body::MJBody;
use super::head::mj_head::MJHead;
use super::prelude::*;
use super::Error;
use crate::parser::{Element, Node};
use crate::util::context::Context;
use crate::util::header::Header;
use crate::Options;
use log::debug;

#[derive(Clone, Debug)]
pub struct MJMLElement<'a> {
    context: Option<Context>,
    head: MJHead<'a>,
    body: MJBody,
}

fn get_head<'a>(node: &Node<'a>, opts: Options) -> Result<MJHead<'a>, Error> {
    for child in node.children.iter() {
        match child {
            Element::Node(node) => {
                if node.name.as_str() == "mj-head" {
                    return MJHead::parse(&node, opts);
                }
            }
            _ => (),
        }
    }
    Ok(MJHead::empty(opts))
}

fn get_body<'a>(node: &Node<'a>, header: &Header) -> Result<MJBody, Error> {
    for child in node.children.iter() {
        match child {
            Element::Node(node) => {
                if node.name.as_str() == "mj-body" {
                    return MJBody::parse(&node, &header);
                }
            }
            _ => (),
        }
    }
    Ok(MJBody::empty())
}

impl<'a> MJMLElement<'a> {
    pub fn parse(node: &Node<'a>, opts: Options) -> Result<MJMLElement<'a>, Error> {
        let mut head = get_head(node, opts)?;
        let mut body = get_body(node, head.get_header())?;
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

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;
    use crate::{to_html, Options};

    #[test]
    fn basic() {
        compare_render(
            include_str!("../../test/mjml.mjml"),
            include_str!("../../test/mjml.html"),
        );
    }

    #[test]
    fn debug() {
        let template = include_str!("../../test/mjml.mjml");
        let result = to_html(template, Options::default()).unwrap();
        println!("result: {:?}", result);
    }
}
