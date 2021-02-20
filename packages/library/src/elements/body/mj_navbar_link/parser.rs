use super::MJNavbarLink;
use crate::elements::error::Error;
use crate::parser::{MJMLParser, Node};
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("color", "#000000")
        .add("font-family", "Ubuntu, Helvetica, Arial, sans-serif")
        .add("font-size", "13px")
        .add("font-weight", "normal")
        .add("line-height", "22px")
        .add("padding", "15px 10px")
        .add("target", "_blank")
        .add("text-decoration", "none")
        .add("text-transform", "uppercase");
}

struct MJNavbarLinkParser<'h> {
    header: &'h Header,
    extra: Attributes,
    attributes: Attributes,
    content: Option<String>,
}

impl<'h> MJNavbarLinkParser<'h> {
    pub fn new(header: &'h Header, extra: Option<&Attributes>) -> Self {
        Self {
            header,
            extra: extra.cloned().unwrap_or_default(),
            attributes: Attributes::new(),
            content: None,
        }
    }

    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse_link<'a>(mut self, node: &Node<'a>) -> Result<Self, Error> {
        if node.name.as_str() != "mj-navbar-link" {
            return Err(Error::UnexpectedElement(node.name.as_str().into()));
        }
        let content = node
            .children
            .iter()
            .filter_map(|child| child.as_text())
            .map(|child| child.as_str())
            .collect::<String>();
        self.attributes = Self::default_attributes(node, self.header);
        self.attributes.merge(&self.extra);
        self.attributes.merge(node);
        self.content = if content.is_empty() {
            None
        } else {
            Some(content)
        };
        Ok(self)
    }
}

impl<'h> MJMLParser for MJNavbarLinkParser<'h> {
    type Output = MJNavbarLink;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJNavbarLink {
            attributes: self.attributes,
            context: None,
            content: self.content,
        })
    }

    fn parse<'a>(mut self, node: &Node<'a>) -> Result<Self, Error> {
        if self.extra.get("text-padding").is_none() {
            self.extra.set("text-padding", "4px 4px 4px 0");
        }
        self.parse_link(node)
    }
}

impl MJNavbarLink {
    pub fn parse<'a>(
        node: &Node<'a>,
        header: &Header,
        extra: Option<&Attributes>,
    ) -> Result<MJNavbarLink, Error> {
        MJNavbarLinkParser::new(header, extra).parse(node)?.build()
    }

    pub fn parse_link<'a>(
        node: &Node<'a>,
        header: &Header,
        extra: Option<&Attributes>,
    ) -> Result<MJNavbarLink, Error> {
        MJNavbarLinkParser::new(header, extra)
            .parse_link(node)?
            .build()
    }
}
