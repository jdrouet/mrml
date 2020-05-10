use crate::util::Properties;
use roxmltree::Node;

pub mod error;
mod mj_body;
mod mj_head;
mod mj_section;
mod mjml;
pub mod prelude;
mod raw;

use error::Error;
use prelude::Component;

pub enum Element<'a, 'b> {
    MJML(mjml::MJMLElement<'a, 'b>),
    MJBody(mj_body::MJBody<'a, 'b>),
    MJHead(mj_head::MJHead<'a, 'b>),
    MJSection(mj_section::MJSection<'a, 'b>),
    Raw(raw::RawElement<'a, 'b>),
}

impl Component for Element<'_, '_> {
    fn default_attribute(_key: &str) -> Option<String> {
        None
    }

    fn node(&self) -> Option<Node> {
        None
    }

    fn set_context(&mut self, ctx: Properties) {
        match self {
            Element::MJML(item) => item.set_context(ctx),
            Element::MJBody(item) => item.set_context(ctx),
            Element::MJHead(item) => item.set_context(ctx),
            Element::MJSection(item) => item.set_context(ctx),
            Element::Raw(item) => item.set_context(ctx),
        }
    }

    fn render(&self) -> Result<String, Error> {
        match self {
            Element::MJML(item) => item.render(),
            Element::MJBody(item) => item.render(),
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
            "mj-head" => Element::MJHead(mj_head::MJHead::parse(node)?),
            "mj-section" => Element::MJSection(mj_section::MJSection::parse(node)?),
            _ => Element::Raw(raw::RawElement::parse(node)?),
        };
        Ok(res)
    }

    pub fn digest(&mut self) {
        // match self {
        //     Element::MJML(item) => item.render(),
        //     Element::MJBody(item) => item.render(),
        //     Element::MJHead(item) => item.render(),
        //     Element::MJSection(item) => item.render(),
        //     Element::Raw(item) => item.render(),
        //     _ => (),
        // };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use roxmltree::Document;

    #[test]
    fn parse_mjml() {
        let expected = include_str!("../../test/mjml.html");
        let expected = expected.replace("\n", "").replace(" ", "");
        let doc = Document::parse("<mjml></mjml>").unwrap();
        let root = doc.root_element();
        let element = Element::parse(root);
        assert_eq!(element.is_ok(), true);
        let result = element.unwrap().render();
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap().replace("\n", "").replace(" ", "");
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_mj_body() {
        let expected = include_str!("../../test/mj-body.html");
        let expected = expected.replace("\n", "").replace(" ", "");
        let doc = Document::parse("<mjml><mj-body></mj-body></mjml>").unwrap();
        let root = doc.root_element();
        let element = Element::parse(root);
        assert_eq!(element.is_ok(), true);
        let result = element.unwrap().render();
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap().replace("\n", "").replace(" ", "");
        assert_eq!(result, expected);
    }
}
