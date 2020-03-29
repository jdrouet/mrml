use super::{MJCarouselImage, NAME as MJ_CAROUSEL_IMAGE};
use crate::elements::error::Error;
use crate::parser::Node;
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default().add("target", "_blank");
}

impl MJCarouselImage {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse_image<'a>(
        node: &Node<'a>,
        header: &Header,
        extra: Option<&Attributes>,
    ) -> Result<MJCarouselImage, Error> {
        if node.name.as_str() != MJ_CAROUSEL_IMAGE {
            return Err(Error::UnexpectedElement(node.name.as_str().into()));
        }
        let carousel_id = match extra.and_then(|attrs| attrs.get("carousel-id")) {
            Some(id) => id,
            None => return Err(Error::MissingAttribute("carousel-id".into())),
        };
        let content: Vec<&str> = node
            .children
            .iter()
            .filter_map(|child| child.as_text())
            .map(|value| value.as_str())
            .collect();
        let content = if content.is_empty() {
            None
        } else {
            Some(content.join(""))
        };
        let mut attributes = Self::default_attributes(node, header);
        if let Some(extra) = extra {
            attributes.merge(extra);
        }
        attributes.merge(node);
        Ok(MJCarouselImage {
            attributes,
            carousel_id: carousel_id.to_string(),
            context: None,
            content,
        })
    }

    pub fn parse<'a>(
        node: &Node<'a>,
        header: &Header,
        extra: Option<&Attributes>,
    ) -> Result<MJCarouselImage, Error> {
        let mut attrs = match extra {
            Some(value) => value.into(),
            None => Attributes::default(),
        };
        if attrs.get("text-padding").is_none() {
            attrs.set("text-padding", "4px 4px 4px 0");
        }
        Self::parse_image(node, header, Some(&attrs))
    }
}
