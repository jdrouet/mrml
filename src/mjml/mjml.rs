use super::body::mj_body::MJBody;
use super::head::mj_head::MJHead;
use super::Error;
use super::prelude::*;
use crate::util::Context;
use log::debug;
use roxmltree::Node;
use std::collections::HashMap;

const DOCTYPE: &str = "<!doctype html>";
const HTML_OPEN: &str = "<html xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:v=\"urn:schemas-microsoft-com:vml\" xmlns:o=\"urn:schemas-microsoft-com:office:office\">";
const HTML_CLOSE: &str = "</html>";

#[derive(Clone, Debug)]
pub struct MJMLElement {
    context: Option<Context>,
    head: MJHead,
    body: MJBody,
}

fn get_head<'a, 'b>(node: Node<'a, 'b>) -> Result<MJHead, Error> {
    for child in node.children() {
        if child.tag_name().name() == "mj-head" {
            return MJHead::parse(child);
        }
    }
    Ok(MJHead::empty())
}

fn get_body<'a, 'b>(node: Node<'a, 'b>) -> Result<MJBody, Error> {
    for child in node.children() {
        if child.tag_name().name() == "mj-body" {
            return MJBody::parse(child);
        }
    }
    Ok(MJBody::empty())
}

impl MJMLElement {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>) -> Result<MJMLElement, Error> {
        debug!("parse");
        let head = get_head(node)?;
        let body = get_body(node)?;
        let element = MJMLElement {
            context: None,
            head,
            body,
        };
        Ok(element)
    }
}

impl Component for MJMLElement {
    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx.clone());
        self.body.set_context(ctx.clone());
        self.head.set_context(ctx.clone());
    }

    fn render(&self) -> Result<String, Error> {
        debug!("render");
        let mut head = self.head.clone();
        head.set_header(self.body.to_header());
        let mut res: Vec<String> = vec![];
        res.push(DOCTYPE.into());
        res.push(HTML_OPEN.into());
        res.push(head.render()?);
        res.push(self.body.render()?);
        res.push(HTML_CLOSE.into());
        Ok(res.join(""))
    }
}

impl ComponentWithAttributes for MJMLElement {
    fn source_attributes(&self) -> Option<&HashMap<String, String>> {
        None
    }
}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn basic() {
        compare_render(
            include_str!("../../test/mjml.mjml"),
            include_str!("../../test/mjml.html"),
        );
    }
}
