use crate::mjml::error::Error;
use crate::util::Header;
use crate::Options;
use roxmltree::Node;

pub mod mj_breakpoint;
pub mod mj_font;
pub mod mj_head;
pub mod mj_title;
pub mod mj_style;
pub mod prelude;

#[derive(Clone, Debug)]
pub enum HeadElement {
    MJBreakpoint(mj_breakpoint::MJBreakpoint),
    MJFont(mj_font::MJFont),
    MJStyle(mj_style::MJStyle),
    MJTitle(mj_title::MJTitle),
}

macro_rules! apply_fn {
    ($root:expr, $func:ident($($args:tt)*)) => {
        match $root {
            HeadElement::MJBreakpoint(item) => item.$func($($args)*),
            HeadElement::MJFont(item) => item.$func($($args)*),
            HeadElement::MJStyle(item) => item.$func($($args)*),
            HeadElement::MJTitle(item) => item.$func($($args)*),
        }
    };
}

impl HeadElement {
    pub fn parse_all<'a, 'b>(
        nodes: Vec<Node<'a, 'b>>,
        opts: &Options,
    ) -> Result<Vec<HeadElement>, Error> {
        let mut res = vec![];
        for item in nodes {
            if !item.tag_name().name().is_empty() {
                res.push(HeadElement::parse(item, opts)?);
            }
        }
        Ok(res)
    }

    pub fn parse<'a, 'b>(node: Node<'a, 'b>, opts: &Options) -> Result<HeadElement, Error> {
        let tag_name = node.tag_name().name();
        let res = match tag_name {
            "mj-breakpoint" => {
                HeadElement::MJBreakpoint(mj_breakpoint::MJBreakpoint::parse(node, opts)?)
            }
            "mj-font" => HeadElement::MJFont(mj_font::MJFont::parse(node, opts)?),
            "mj-style" => HeadElement::MJStyle(mj_style::MJStyle::parse(node, opts)?),
            "mj-title" => HeadElement::MJTitle(mj_title::MJTitle::parse(node, opts)?),
            _ => return Err(Error::ParseError(format!("{} tag not known", tag_name))),
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
