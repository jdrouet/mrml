use super::{MJAccordionTitle, NAME as MJ_ACCORDION_TITLE};
use crate::elements::error::Error;
use crate::parser::{MJMLParser, Node};
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("font-size", "13px")
        .add("padding", "16px");
}

struct MJAccordionTitleParser<'h, 'p> {
    header: &'h Header,
    parent_attributes: &'p Attributes,
    attributes: Attributes,
    content: String,
}

impl<'h, 'p> MJAccordionTitleParser<'h, 'p> {
    pub fn new(header: &'h Header, parent_attributes: &'p Attributes) -> Self {
        Self {
            header,
            parent_attributes,
            attributes: Attributes::default(),
            content: String::default(),
        }
    }

    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }
}

impl<'h, 'p> MJMLParser for MJAccordionTitleParser<'h, 'p> {
    type Output = MJAccordionTitle;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJAccordionTitle {
            attributes: self.attributes,
            context: None,
            content: self.content,
        })
    }

    fn parse<'a>(mut self, node: &Node<'a>) -> Result<Self, Error> {
        if node.name.as_str() != MJ_ACCORDION_TITLE {
            return Err(Error::UnexpectedElement(node.name.as_str().into()));
        }
        self.attributes = Self::default_attributes(node, self.header)
            .concat(self.parent_attributes)
            .concat(node);
        self.content = node
            .children
            .iter()
            .filter_map(|child| child.as_text())
            .map(|value| value.as_str())
            .collect::<String>();
        Ok(self)
    }
}

impl MJAccordionTitle {
    pub fn parse<'a>(
        node: &Node<'a>,
        header: &Header,
        attrs: &Attributes,
    ) -> Result<MJAccordionTitle, Error> {
        MJAccordionTitleParser::new(header, attrs)
            .parse(node)?
            .build()
    }
}
