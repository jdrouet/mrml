use super::MJCarousel;
use crate::elements::body::mj_carousel_image::MJCarouselImage;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::parser::Node;
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

impl MJCarousel {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJCarousel, Error> {
        let generate: IdGenerator = header.id_generator;
        let mut result = MJCarousel {
            attributes: Self::default_attributes(node, header).concat(node),
            context: None,
            children: vec![],
            id: generate(8),
        };
        let attrs = result.get_children_attributes();
        for child in node.children.iter() {
            if let Some(child_node) = child.as_node() {
                let tag_name = child_node.name.as_str();
                if tag_name != "mj-carousel-image" {
                    return Err(Error::UnexpectedElement(tag_name.into()));
                } else {
                    let element = MJCarouselImage::parse(&child_node, header, Some(&attrs))?;
                    result.children.push(BodyElement::MJCarouselImage(element));
                }
            }
        }
        Ok(result)
    }
}
