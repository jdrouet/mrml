use super::MJNavbarLink;
use crate::elements::error::Error;
use crate::parser::Node;
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

impl MJNavbarLink {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse_link<'a>(
        node: &Node<'a>,
        header: &Header,
        extra: Option<&Attributes>,
    ) -> Result<MJNavbarLink, Error> {
        if node.name.as_str() != "mj-navbar-link" {
            return Err(Error::UnexpectedElement(node.name.as_str().into()));
        }
        let content = node
            .children
            .iter()
            .filter_map(|child| child.as_text())
            .map(|child| child.as_str())
            .collect::<String>();
        let mut attributes = Self::default_attributes(node, header);
        if let Some(extra) = extra {
            attributes.merge(extra);
        }
        attributes.merge(node);
        Ok(MJNavbarLink {
            attributes,
            context: None,
            content: if content.is_empty() {
                None
            } else {
                Some(content)
            },
        })
    }

    pub fn parse<'a>(
        node: &Node<'a>,
        header: &Header,
        extra: Option<&Attributes>,
    ) -> Result<MJNavbarLink, Error> {
        let mut attrs = match extra {
            Some(value) => value.into(),
            None => Attributes::default(),
        };
        if attrs.get("text-padding").is_none() {
            attrs.set("text-padding", "4px 4px 4px 0");
        }
        Self::parse_link(node, header, Some(&attrs))
    }
}
