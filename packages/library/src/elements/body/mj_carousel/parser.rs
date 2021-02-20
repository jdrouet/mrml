use super::MJCarousel;
use crate::elements::body::mj_carousel_image::{MJCarouselImage, NAME as MJ_CAROUSEL_IMAGE};
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::parser::{MJMLParser, Node};
use crate::util::attributes::*;
use crate::util::header::Header;
use crate::util::id::Generator as IdGenerator;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("align", "center")
        .add("border-radius", "6px")
        .add("icon-width", "44px")
        .add("left-icon", "https://i.imgur.com/xTh3hln.png")
        .add("right-icon", "https://i.imgur.com/os7o9kz.png")
        .add("thumbnails", "visible")
        .add("tb-border", "2px solid transparent")
        .add("tb-border-radius", "6px")
        .add("tb-hover-border-color", "#fead0d")
        .add("tb-selected-border-color", "#cccccc");
}

struct MJCarouselParser<'h> {
    header: &'h Header,
    result: Option<MJCarousel>,
}

impl<'h> MJCarouselParser<'h> {
    pub fn new(header: &'h Header) -> Self {
        Self {
            header,
            result: None,
        }
    }

    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }
}

impl<'h> MJMLParser for MJCarouselParser<'h> {
    type Output = MJCarousel;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.result.unwrap())
    }

    fn parse<'a>(mut self, node: &Node<'a>) -> Result<Self, Error> {
        let generate: IdGenerator = self.header.id_generator;
        let mut result = MJCarousel {
            attributes: Self::default_attributes(node, self.header).concat(node),
            context: None,
            children: vec![],
            id: generate(8),
        };
        let attrs = result.get_children_attributes();
        for child in node.children.iter() {
            if let Some(child_node) = child.as_node() {
                let tag_name = child_node.name.as_str();
                if tag_name != MJ_CAROUSEL_IMAGE {
                    return Err(Error::UnexpectedElement(tag_name.into()));
                } else {
                    let element = MJCarouselImage::parse(&child_node, self.header, Some(&attrs))?;
                    result.children.push(BodyElement::MJCarouselImage(element));
                }
            }
        }
        self.result = Some(result);
        Ok(self)
    }
}

impl MJCarousel {
    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJCarousel, Error> {
        MJCarouselParser::new(header).parse(node)?.build()
    }
}
