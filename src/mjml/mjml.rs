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
    options: Options,
    context: Option<Context>,
    head: MJHead,
    body: MJBody,
}

fn get_head<'a, 'b>(node: Node<'a, 'b>, opts: &Options) -> Result<MJHead, Error> {
    for child in node.children() {
        if child.tag_name().name() == "mj-head" {
            return MJHead::parse(child, &opts);
        }
    }
    Ok(MJHead::empty(&opts))
}

fn get_body<'a, 'b>(node: Node<'a, 'b>, opts: &Options) -> Result<MJBody, Error> {
    for child in node.children() {
        if child.tag_name().name() == "mj-body" {
            return MJBody::parse(child, opts);
        }
    }
    Ok(MJBody::empty(opts))
}

impl MJMLElement {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>, opts: &Options) -> Result<MJMLElement, Error> {
        let head = get_head(node, opts)?;
        let body = get_body(node, opts)?;
        let mut element = MJMLElement {
            options: opts.clone(),
            context: None,
            head,
            body,
        };
        element.digest();
        Ok(element)
    }

    pub fn digest(&mut self) {
        self.body.set_context(Context::default());
    }

    pub fn get_header(&self) -> Header {
        let mut header = Header::from(&self.options);
        self.head.update_header(&mut header);
        self.body.update_header(&mut header);
        header
    }

    pub fn get_title(&self, header: &Header) -> String {
        debug!("get_title");
        self.head.get_title(header)
    }

    pub fn get_preview(&self, header: &Header) -> String {
        debug!("get_preview");
        self.head.get_preview(header)
    }

    pub fn get_html(&self, header: &Header) -> Result<String, Error> {
        debug!("get_html");
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
