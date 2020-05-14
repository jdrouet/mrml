use crate::util::{Context, Header, Style};
use roxmltree::Node;

pub mod error;
mod mj_body;
mod mj_column;
mod mj_head;
mod mj_section;
mod mjml;
pub mod prelude;
mod raw;

use error::Error;
use prelude::Component;

#[derive(Clone, Debug)]
pub enum Element<'a, 'b> {
    MJML(mjml::MJMLElement<'a, 'b>),
    MJBody(mj_body::MJBody<'a, 'b>),
    MJColumn(mj_column::MJColumn<'a, 'b>),
    MJHead(mj_head::MJHead<'a, 'b>),
    MJSection(mj_section::MJSection<'a, 'b>),
    Raw(raw::RawElement<'a, 'b>),
}

impl Component for Element<'_, '_> {
    fn node(&self) -> Option<Node> {
        match self {
            Element::MJML(item) => item.node(),
            Element::MJBody(item) => item.node(),
            Element::MJColumn(item) => item.node(),
            Element::MJHead(item) => item.node(),
            Element::MJSection(item) => item.node(),
            Element::Raw(item) => item.node(),
        }
    }

    fn to_header(&self) -> Header {
        match self {
            Element::MJML(item) => item.to_header(),
            Element::MJBody(item) => item.to_header(),
            Element::MJColumn(item) => item.to_header(),
            Element::MJHead(item) => item.to_header(),
            Element::MJSection(item) => item.to_header(),
            Element::Raw(item) => item.to_header(),
        }
    }

    fn context(&self) -> Option<&Context> {
        match self {
            Element::MJML(item) => item.context(),
            Element::MJBody(item) => item.context(),
            Element::MJColumn(item) => item.context(),
            Element::MJHead(item) => item.context(),
            Element::MJSection(item) => item.context(),
            Element::Raw(item) => item.context(),
        }
    }

    fn get_style(&self, key: &str) -> Style {
        match self {
            Element::MJML(item) => item.get_style(key),
            Element::MJBody(item) => item.get_style(key),
            Element::MJColumn(item) => item.get_style(key),
            Element::MJHead(item) => item.get_style(key),
            Element::MJSection(item) => item.get_style(key),
            Element::Raw(item) => item.get_style(key),
        }
    }

    fn set_context(&mut self, ctx: Context) {
        match self {
            Element::MJML(item) => item.set_context(ctx),
            Element::MJBody(item) => item.set_context(ctx),
            Element::MJColumn(item) => item.set_context(ctx),
            Element::MJHead(item) => item.set_context(ctx),
            Element::MJSection(item) => item.set_context(ctx),
            Element::Raw(item) => item.set_context(ctx),
        }
    }

    fn render(&self) -> Result<String, Error> {
        match self {
            Element::MJML(item) => item.render(),
            Element::MJBody(item) => item.render(),
            Element::MJColumn(item) => item.render(),
            Element::MJHead(item) => item.render(),
            Element::MJSection(item) => item.render(),
            Element::Raw(item) => item.render(),
        }
    }
}

impl Element<'_, '_> {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>) -> Result<Element<'a, 'b>, Error> {
        let res = match node.tag_name().name() {
            "mjml" => Element::MJML(mjml::MJMLElement::parse(node)?),
            "mj-body" => Element::MJBody(mj_body::MJBody::parse(node)?),
            "mj-column" => Element::MJColumn(mj_column::MJColumn::parse(node)?),
            "mj-head" => Element::MJHead(mj_head::MJHead::parse(node)?),
            "mj-section" => Element::MJSection(mj_section::MJSection::parse(node)?),
            _ => Element::Raw(raw::RawElement::parse(node)?),
        };
        Ok(res)
    }
}
