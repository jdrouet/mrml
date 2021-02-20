use super::{MJWrapper, DEFAULT_BACKGROUND_POSITION};
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::parser::{MJMLParser, Node};
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("background-position", DEFAULT_BACKGROUND_POSITION)
        .add("background-repeat", "repeat")
        .add("background-size", "auto")
        .add("direction", "ltr")
        .add("padding", "20px 0")
        .add("text-align", "center")
        .add("text-padding", "4px 4px 4px 0");
}

struct MJWrapperParser<'h> {
    header: &'h Header,
    attributes: Attributes,
    children: Vec<BodyElement>,
}

impl<'h> MJWrapperParser<'h> {
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

impl<'h> MJMLParser for MJWrapperParser<'h> {
    type Output = MJWrapper;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJWrapper {
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

impl MJWrapper {
    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJWrapper, Error> {
        MJWrapperParser::new(header).parse(node)?.build()
    }
}
