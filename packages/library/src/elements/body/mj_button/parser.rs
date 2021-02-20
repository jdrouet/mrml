use super::MJButton;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::parser::{MJMLParser, Node};
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("align", "center")
        .add("background-color", "#414141")
        .add("border", "none")
        .add("border-radius", "3px")
        .add("color", "#ffffff")
        .add("font-family", "Ubuntu, Helvetica, Arial, sans-serif")
        .add("font-size", "13px")
        .add("font-weight", "normal")
        .add("inner-padding", "10px 25px")
        .add("line-height", "120%")
        .add("padding", "10px 25px")
        .add("target", "_blank")
        .add("text-decoration", "none")
        .add("text-transform", "none")
        .add("vertical-align", "middle");
}

struct MJButtonParser<'h> {
    header: &'h Header,
    attributes: Attributes,
    children: Vec<BodyElement>,
}

impl<'h> MJButtonParser<'h> {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn new(header: &'h Header) -> Self {
        Self {
            header,
            attributes: Attributes::new(),
            children: vec![],
        }
    }
}

impl<'h> MJMLParser for MJButtonParser<'h> {
    type Output = MJButton;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJButton {
            attributes: self.attributes,
            context: None,
            children: self.children,
        })
    }

    fn parse<'a>(mut self, node: &Node<'a>) -> Result<Self, Error> {
        self.attributes = Self::default_attributes(node, self.header).concat(node);
        for child in node.children.iter() {
            self.children.push(BodyElement::parse(
                &child,
                self.header,
                None::<&Attributes>,
            )?);
        }
        Ok(self)
    }
}

impl MJButton {
    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJButton, Error> {
        MJButtonParser::new(header).parse(node)?.build()
    }
}
