use super::{MJAccordionText, NAME as MJ_ACCORDION_TEXT};
use crate::elements::body::raw::RawElement;
use crate::elements::error::Error;
use crate::parser::{MJMLParser, Node};
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("line-height", "1")
        .add("font-size", "13px")
        .add("padding", "16px");
}

struct MJAccordionTextParser<'h, 'p> {
    header: &'h Header,
    parent_attributes: &'p Attributes,
    attributes: Attributes,
    children: Vec<RawElement>,
}

impl<'h, 'p> MJAccordionTextParser<'h, 'p> {
    pub fn new(header: &'h Header, parent_attributes: &'p Attributes) -> Self {
        Self {
            header,
            parent_attributes,
            attributes: Attributes::default(),
            children: Vec::new(),
        }
    }

    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }
}

impl<'h, 'p> MJMLParser for MJAccordionTextParser<'h, 'p> {
    type Output = MJAccordionText;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJAccordionText {
            attributes: self.attributes,
            context: None,
            children: self.children,
        })
    }

    fn parse<'a>(mut self, node: &Node<'a>) -> Result<Self, Error> {
        if node.name.as_str() != MJ_ACCORDION_TEXT {
            return Err(Error::UnexpectedElement(node.name.as_str().into()));
        }
        self.attributes = Self::default_attributes(node, self.header)
            .concat(self.parent_attributes)
            .concat(node);
        for child in node.children.iter() {
            self.children
                .push(RawElement::conditional_parse(&child, self.header, true)?);
        }
        Ok(self)
    }
}

impl MJAccordionText {
    pub fn parse<'a>(
        node: &Node<'a>,
        header: &Header,
        attrs: &Attributes,
    ) -> Result<MJAccordionText, Error> {
        MJAccordionTextParser::new(header, attrs)
            .parse(node)?
            .build()
    }
}
