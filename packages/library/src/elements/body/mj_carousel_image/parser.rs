use super::{MJCarouselImage, NAME as MJ_CAROUSEL_IMAGE};
use crate::elements::error::Error;
use crate::parser::{MJMLParser, Node};
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default().add("target", "_blank");
}

struct MJCarouselImageParser<'h> {
    header: &'h Header,
    parent_attributes: Attributes,
    carousel_id: String,
    attributes: Attributes,
    content: Option<String>,
}

impl<'h> MJCarouselImageParser<'h> {
    pub fn new(header: &'h Header, extra: Attributes) -> Self {
        Self {
            header,
            parent_attributes: extra,
            carousel_id: String::default(),
            attributes: Attributes::default(),
            content: None,
        }
    }

    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    fn parse_image<'a>(mut self, node: &Node<'a>) -> Result<Self, Error> {
        if node.name.as_str() != MJ_CAROUSEL_IMAGE {
            return Err(Error::UnexpectedElement(node.name.as_str().into()));
        }
        let carousel_id = match self.parent_attributes.get("carousel-id") {
            Some(id) => id,
            None => return Err(Error::MissingAttribute("carousel-id".into())),
        };
        let content = node
            .children
            .iter()
            .filter_map(|child| child.as_text())
            .map(|value| value.to_string())
            .collect::<String>();
        self.content = if content.is_empty() {
            None
        } else {
            Some(content)
        };
        self.attributes = Self::default_attributes(node, self.header)
            .concat(&self.parent_attributes)
            .concat(node);
        Ok(self)
    }
}

impl<'h> MJMLParser for MJCarouselImageParser<'h> {
    type Output = MJCarouselImage;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJCarouselImage {
            attributes: self.attributes,
            carousel_id: self.carousel_id,
            context: None,
            content: self.content,
        })
    }

    fn parse<'a>(mut self, node: &Node<'a>) -> Result<Self, Error> {
        if self.parent_attributes.get("text-padding").is_none() {
            self.parent_attributes.set("text-padding", "4px 4px 4px 0");
        }
        self.parse_image(node)
    }
}

impl MJCarouselImage {
    pub fn parse<'a>(
        node: &Node<'a>,
        header: &Header,
        extra: Option<&Attributes>,
    ) -> Result<MJCarouselImage, Error> {
        MJCarouselImageParser::new(header, extra.cloned().unwrap_or_default())
            .parse(node)?
            .build()
    }
}
