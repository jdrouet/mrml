use crate::elements::error::Error;
use crate::parser::{Element, Node};
use crate::util::header::Header;

pub mod mj_attributes;
pub mod mj_breakpoint;
pub mod mj_font;
pub mod mj_head;
pub mod mj_preview;
pub mod mj_style;
pub mod mj_title;
pub mod prelude;

#[derive(Clone, Debug)]
pub enum HeadElement {
    MJAttributes(mj_attributes::MJAttributes),
    MJBreakpoint(mj_breakpoint::MJBreakpoint),
    MJFont(mj_font::MJFont),
    MJPreview(mj_preview::MJPreview),
    MJStyle(mj_style::MJStyle),
    MJTitle(mj_title::MJTitle),
}

macro_rules! apply_fn {
    ($root:expr, $func:ident($($args:tt)*)) => {
        match $root {
            HeadElement::MJAttributes(item) => item.$func($($args)*),
            HeadElement::MJBreakpoint(item) => item.$func($($args)*),
            HeadElement::MJFont(item) => item.$func($($args)*),
            HeadElement::MJPreview(item) => item.$func($($args)*),
            HeadElement::MJStyle(item) => item.$func($($args)*),
            HeadElement::MJTitle(item) => item.$func($($args)*),
        }
    };
}

impl HeadElement {
    pub fn parse_all<'a>(elements: &Vec<Element<'a>>) -> Result<Vec<HeadElement>, Error> {
        let mut res = vec![];
        for elt in elements {
            match elt {
                Element::Node(node) => {
                    res.push(HeadElement::parse(&node)?);
                }
                Element::Comment(_) => (),
                _ => return Err(Error::UnexpectedText),
            }
        }
        Ok(res)
    }

    pub fn parse<'a>(node: &Node<'a>) -> Result<HeadElement, Error> {
        let tag_name = node.name.as_str();
        let res = match tag_name {
            "mj-attributes" => HeadElement::MJAttributes(mj_attributes::MJAttributes::parse(node)?),
            "mj-breakpoint" => HeadElement::MJBreakpoint(mj_breakpoint::MJBreakpoint::parse(node)?),
            "mj-font" => HeadElement::MJFont(mj_font::MJFont::parse(node)?),
            "mj-preview" => HeadElement::MJPreview(mj_preview::MJPreview::parse(node)?),
            "mj-style" => HeadElement::MJStyle(mj_style::MJStyle::parse(node)?),
            "mj-title" => HeadElement::MJTitle(mj_title::MJTitle::parse(node)?),
            _ => return Err(Error::UnexpectedElement(tag_name.into())),
        };
        Ok(res)
    }
}

impl prelude::HeadComponent for HeadElement {
    fn update_header(&self, header: &mut Header) {
        apply_fn!(self, update_header(header));
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{to_html, Options};

    #[test]
    fn unknown_tag() {
        let content = include_str!("../../../test/mj-head-unknown-tag.mjml");
        let result = to_html(content, Options::default());
        assert_eq!(result.is_err(), true);
        let error = result.unwrap_err();
        assert_eq!(error.is_mjml_error(), true);
    }
}
