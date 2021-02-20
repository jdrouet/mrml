use super::MJHero;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::parser::{MJMLParser, Node};
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("background-color", "#ffffff")
        .add("background-position", "center center")
        .add("height", "0px")
        .add("mode", "fixed-height")
        .add("padding", "0px")
        .add("vertical-align", "top");
}

struct MJHeroParser<'h> {
    header: &'h Header,
    attributes: Attributes,
    children: Vec<BodyElement>,
}

impl<'h> MJHeroParser<'h> {
    pub fn new(header: &'h Header) -> Self {
        Self {
            header,
            attributes: Attributes::new(),
            children: Vec::new(),
        }
    }

    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }
}

impl<'h> MJMLParser for MJHeroParser<'h> {
    type Output = MJHero;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJHero {
            attributes: self.attributes,
            context: None,
            children: self.children,
        })
    }

    fn parse<'a>(mut self, node: &Node<'a>) -> Result<Self, Error> {
        self.attributes = Self::default_attributes(node, self.header).concat(node);
        for child in node.children.iter() {
            self.children
                .push(BodyElement::parse(&child, self.header, None)?);
        }
        Ok(self)
    }
}

impl MJHero {
    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJHero, Error> {
        MJHeroParser::new(header).parse(node)?.build()
    }
}
