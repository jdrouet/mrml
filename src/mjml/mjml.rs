use super::body::mj_body::MJBody;
use super::head::mj_head::MJHead;
use super::prelude::*;
use super::Error;
use crate::util::{Context, Header};
use crate::Options;
use log::debug;
use roxmltree::Node;

const DOCTYPE: &str = "<!doctype html>";
const HTML_OPEN: &str = "<html xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:v=\"urn:schemas-microsoft-com:vml\" xmlns:o=\"urn:schemas-microsoft-com:office:office\">";
const HTML_CLOSE: &str = "</html>";

#[derive(Clone, Debug)]
pub struct MJMLElement {
    context: Option<Context>,
    head: MJHead,
    body: MJBody,
}

fn get_head<'a, 'b>(node: &Node<'a, 'b>, opts: &Options) -> Result<MJHead, Error> {
    for child in node.children() {
        if child.tag_name().name() == "mj-head" {
            return MJHead::parse(&child, &opts);
        }
    }
    Ok(MJHead::empty(&opts))
}

fn get_body<'a, 'b>(node: &Node<'a, 'b>, header: &Header) -> Result<MJBody, Error> {
    for child in node.children() {
        if child.tag_name().name() == "mj-body" {
            return MJBody::parse(&child, header);
        }
    }
    Ok(MJBody::empty(header))
}

impl MJMLElement {
    pub fn parse<'a, 'b>(node: &Node<'a, 'b>, opts: &Options) -> Result<MJMLElement, Error> {
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
        let mut res: Vec<String> = vec![];
        res.push(DOCTYPE.into());
        res.push(HTML_OPEN.into());
        res.push(self.head.render(&header)?);
        res.push(self.body.render(&header)?);
        res.push(HTML_CLOSE.into());
        Ok(res.join(""))
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
