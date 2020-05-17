use crate::util::{Context, Header, Style};
use roxmltree::Node;
use std::collections::HashMap;

pub mod error;
mod mj_body;
mod mj_column;
mod mj_head;
mod mj_image;
mod mj_section;
mod mj_text;
mod mjml;
pub mod prelude;
mod raw;

use error::Error;
use prelude::Component;

#[derive(Clone, Debug)]
pub enum Element {
    MJML(mjml::MJMLElement),
    MJBody(mj_body::MJBody),
    MJColumn(mj_column::MJColumn),
    MJHead(mj_head::MJHead),
    MJImage(mj_image::MJImage),
    MJSection(mj_section::MJSection),
    MJText(mj_text::MJText),
    Raw(raw::RawElement),
}

macro_rules! apply_fn {
    ($root:expr, $func:ident($($args:tt)*)) => {
        match $root {
            Element::MJML(item) => item.$func($($args)*),
            Element::MJBody(item) => item.$func($($args)*),
            Element::MJColumn(item) => item.$func($($args)*),
            Element::MJHead(item) => item.$func($($args)*),
            Element::MJImage(item) => item.$func($($args)*),
            Element::MJSection(item) => item.$func($($args)*),
            Element::MJText(item) => item.$func($($args)*),
            Element::Raw(item) => item.$func($($args)*),
        }
    };
}

impl Component for Element {
    fn source_attributes(&self) -> Option<&HashMap<String, String>> {
        apply_fn!(self, source_attributes())
    }

    fn to_header(&self) -> Header {
        apply_fn!(self, to_header())
    }

    fn context(&self) -> Option<&Context> {
        apply_fn!(self, context())
    }

    fn get_attribute(&self, key: &str) -> Option<String> {
        apply_fn!(self, get_attribute(key))
    }

    fn get_style(&self, key: &str) -> Style {
        apply_fn!(self, get_style(key))
    }

    fn set_context(&mut self, ctx: Context) {
        apply_fn!(self, set_context(ctx))
    }

    fn render(&self) -> Result<String, Error> {
        apply_fn!(self, render())
    }
}

impl Element {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>) -> Result<Element, Error> {
        let res = match node.tag_name().name() {
            "mjml" => Element::MJML(mjml::MJMLElement::parse(node)?),
            "mj-body" => Element::MJBody(mj_body::MJBody::parse(node)?),
            "mj-column" => Element::MJColumn(mj_column::MJColumn::parse(node)?),
            "mj-head" => Element::MJHead(mj_head::MJHead::parse(node)?),
            "mj-image" => Element::MJImage(mj_image::MJImage::parse(node)?),
            "mj-section" => Element::MJSection(mj_section::MJSection::parse(node)?),
            "mj-text" => Element::MJText(mj_text::MJText::parse(node)?),
            _ => Element::Raw(raw::RawElement::parse(node)?),
        };
        Ok(res)
    }
}
