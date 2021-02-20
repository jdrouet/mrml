use super::MJTable;
use crate::elements::body::raw::RawElement;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::parser::{MJMLParser, Node};
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("align", "left")
        .add("border", "none")
        .add("cellpadding", "0")
        .add("cellspacing", "0")
        .add("color", "#000000")
        .add("font-family", "Ubuntu, Helvetica, Arial, sans-serif")
        .add("font-size", "13px")
        .add("line-height", "22px")
        .add("padding", "10px 25px")
        .add("table-layout", "auto")
        .add("width", "100%");
}

struct MJTableParser<'h> {
    header: &'h Header,
    attributes: Attributes,
    children: Vec<BodyElement>,
}

impl<'h> MJTableParser<'h> {
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

impl<'h> MJMLParser for MJTableParser<'h> {
    type Output = MJTable;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJTable {
            attributes: self.attributes,
            context: None,
            children: self.children,
        })
    }

    fn parse<'a>(mut self, node: &Node<'a>) -> Result<Self, Error> {
        self.attributes = Self::default_attributes(node, self.header).concat(node);
        for child in node.children.iter() {
            self.children
                .push(RawElement::conditional_parse(&child, self.header, true)?.into());
        }
        Ok(self)
    }
}

impl MJTable {
    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJTable, Error> {
        MJTableParser::new(header).parse(node)?.build()
    }
}
