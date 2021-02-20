use super::MJAccordion;
use crate::elements::body::mj_accordion_element::{
    MJAccordionElement, NAME as MJ_ACCORDION_ELEMENT,
};
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::parser::{Element, MJMLParser, Node};
use crate::util::attributes::*;
use crate::util::header::Header;

const CHILDREN_ATTRIBUTES: [&str; 9] = [
    "border",
    "icon-align",
    "icon-width",
    "icon-height",
    "icon-position",
    "icon-wrapped-url",
    "icon-wrapped-alt",
    "icon-unwrapped-url",
    "icon-unwrapped-alt",
];

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("border", "2px solid black")
        .add("font-family", "Ubuntu, Helvetica, Arial, sans-serif")
        .add("icon-align", "middle")
        .add("icon-position", "right")
        .add("icon-height", "32px")
        .add("icon-width", "32px")
        .add("icon-wrapped-url", "https://i.imgur.com/bIXv1bk.png")
        .add("icon-wrapped-alt", "+")
        .add("icon-unwrapped-url", "https://i.imgur.com/w4uTygT.png")
        .add("icon-unwrapped-alt", "-")
        .add("padding", "10px 25px");
}

struct MJAccordionParser<'h> {
    header: &'h Header,
    result: Option<MJAccordion>,
}

impl<'h> MJAccordionParser<'h> {
    pub fn new(header: &'h Header) -> Self {
        Self {
            header,
            result: None,
        }
    }

    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    fn get_children_attributes(attrs: &Attributes) -> Attributes {
        let mut res = Attributes::default();
        for key in CHILDREN_ATTRIBUTES.iter() {
            if let Some(value) = attrs.get(key) {
                res.set(key, value);
            }
        }
        res
    }
}

impl<'h> MJMLParser for MJAccordionParser<'h> {
    type Output = MJAccordion;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.result.unwrap())
    }

    fn parse<'a>(mut self, node: &Node<'a>) -> Result<Self, Error> {
        let mut result = MJAccordion {
            attributes: Self::default_attributes(node, self.header).concat(node),
            context: None,
            children: vec![],
        };
        let child_attrs = Self::get_children_attributes(&result.attributes);
        for child in node.children.iter() {
            match child {
                Element::Node(node) => match node.name.as_str() {
                    MJ_ACCORDION_ELEMENT => {
                        let element = MJAccordionElement::parse(node, self.header, &child_attrs)?;
                        result
                            .children
                            .push(BodyElement::MJAccordionElement(element));
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

impl MJAccordion {
    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJAccordion, Error> {
        MJAccordionParser::new(header).parse(node)?.build()
    }
}
