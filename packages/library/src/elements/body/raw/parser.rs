use super::RawElement;
use crate::elements::body::comment::Comment;
use crate::elements::body::node::Node;
use crate::elements::body::text::Text;
use crate::elements::Error;
use crate::parser::Element;
use crate::util::header::Header;

impl RawElement {
    pub fn parse<'a>(element: &Element<'a>, header: &Header) -> Result<RawElement, Error> {
        RawElement::conditional_parse(element, header, false)
    }

    pub fn conditional_parse<'a>(
        element: &Element<'a>,
        header: &Header,
        only_raw: bool,
    ) -> Result<RawElement, Error> {
        match element {
            Element::Text(value) => Ok(RawElement::Text(Text::from(value.to_string()))),
            Element::Comment(value) => Ok(RawElement::Comment(Comment::from(value.to_string()))),
            Element::Node(node) => Ok(RawElement::Node(Node::conditional_parse(
                node, header, only_raw,
            )?)),
        }
    }
}
