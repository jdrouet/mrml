use super::{MJSection, DEFAULT_BACKGROUND_POSITION};
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

struct MJSectionParser<'h> {
    header: &'h Header,
    attributes: Attributes,
    children: Vec<BodyElement>,
}

impl<'h> MJSectionParser<'h> {
    pub fn new(header: &'h Header) -> Self {
        Self {
            header,
            attributes: Attributes::new(),
            children: vec![],
        }
    }

    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }
}

impl<'h> MJMLParser for MJSectionParser<'h> {
    type Output = MJSection;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJSection {
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

impl MJSection {
    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJSection, Error> {
        MJSectionParser::new(header).parse(node)?.build()
    }
}
