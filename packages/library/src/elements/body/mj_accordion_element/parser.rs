use super::{MJAccordionElement, NAME as MJ_ACCORDION_ELEMENT};
use crate::elements::body::mj_accordion_text::{MJAccordionText, NAME as MJ_ACCORDION_TEXT};
use crate::elements::body::mj_accordion_title::{MJAccordionTitle, NAME as MJ_ACCORDION_TITLE};
use crate::elements::error::Error;
use crate::parser::{Element, MJMLParser, Node};
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default();
}

struct MJAccordionElementParser<'h, 'p> {
    header: &'h Header,
    parent_attributes: &'p Attributes,
    result: Option<MJAccordionElement>,
}

impl<'h, 'p> MJAccordionElementParser<'h, 'p> {
    pub fn new(header: &'h Header, parent_attributes: &'p Attributes) -> Self {
        Self {
            header,
            parent_attributes,
            result: None,
        }
    }

    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }
}

impl<'h, 'p> MJMLParser for MJAccordionElementParser<'h, 'p> {
    type Output = MJAccordionElement;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.result.unwrap())
    }

    fn parse<'a>(mut self, node: &Node<'a>) -> Result<Self, Error> {
        if node.name.as_str() != MJ_ACCORDION_ELEMENT {
            return Err(Error::UnexpectedElement(node.name.as_str().into()));
        }
        let mut result = MJAccordionElement {
            attributes: Self::default_attributes(node, self.header)
                .concat(self.parent_attributes)
                .concat(node),
            context: None,
            title: None,
            text: None,
        };
        let children_attr = result.get_children_attributes();
        for child in node.children.iter() {
            match child {
                Element::Node(node) => match node.name.as_str() {
                    MJ_ACCORDION_TITLE => {
                        result.title =
                            Some(MJAccordionTitle::parse(node, self.header, &children_attr)?);
                    }
                    MJ_ACCORDION_TEXT => {
                        result.text =
                            Some(MJAccordionText::parse(node, self.header, &children_attr)?);
                    }
                    name => return Err(Error::UnexpectedElement(name.into())),
                },
                // TODO handle comments
                Element::Comment(_) => (),
                Element::Text(_) => return Err(Error::UnexpectedText),
            };
        }
        self.result = Some(result);
        Ok(self)
    }
}

impl MJAccordionElement {
    pub fn parse<'a>(
        node: &Node<'a>,
        header: &Header,
        attrs: &Attributes,
    ) -> Result<MJAccordionElement, Error> {
        MJAccordionElementParser::new(header, attrs)
            .parse(node)?
            .build()
    }
}
