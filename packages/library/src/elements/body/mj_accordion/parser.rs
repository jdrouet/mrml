use super::{MJAccordion, MJAccordionChild};
use crate::elements::body::generic::ComponentOrComment;
use crate::elements::body::mj_accordion_element::{
    MJAccordionElement, NAME as MJ_ACCORDION_ELEMENT,
};
use crate::parser::{Error, MJMLParser};
use crate::util::attributes::*;
use crate::util::header::Header;
use xmlparser::{StrSpan, Tokenizer};

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
    attributes: Attributes,
    children: Vec<MJAccordionChild>,
}

impl<'h> MJAccordionParser<'h> {
    pub fn new(header: &'h Header) -> Self {
        Self {
            header,
            attributes: Attributes::default(),
            children: Vec::default(),
        }
    }

    fn build_attributes(&self) -> Attributes {
        self.header
            .default_attributes
            .concat_attributes(super::NAME, &DEFAULT_ATTRIBUTES, &self.attributes)
            .concat(&self.attributes)
    }

    fn get_children_attributes(&self) -> Attributes {
        let mut res = Attributes::default();
        let attrs = self.build_attributes();
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
        Ok(MJAccordion {
            attributes: self.build_attributes(),
            context: None,
            children: self.children,
        })
    }

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, value: StrSpan<'a>) -> Result<(), Error> {
        self.attributes.set(name, value);
        Ok(())
    }

    fn parse_child_comment(&mut self, value: StrSpan) -> Result<(), Error> {
        self.children
            .push(ComponentOrComment::comment(value.to_string()));
        Ok(())
    }

    fn parse_child_element<'a>(
        &mut self,
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<(), Error> {
        if tag.as_str() != MJ_ACCORDION_ELEMENT {
            return Err(Error::UnexpectedElement(tag.start()));
        }
        let child_attrs = self.get_children_attributes();
        let element = MJAccordionElement::parse(tokenizer, self.header, &child_attrs)?;
        self.children.push(ComponentOrComment::from(element));
        Ok(())
    }
}

impl MJAccordion {
    pub fn parse<'a>(tokenizer: &mut Tokenizer<'a>, header: &Header) -> Result<MJAccordion, Error> {
        MJAccordionParser::new(header).parse(tokenizer)?.build()
    }
}
