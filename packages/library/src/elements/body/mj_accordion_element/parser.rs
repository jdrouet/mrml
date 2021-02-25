use super::MJAccordionElement;
use crate::elements::body::mj_accordion_text::{MJAccordionText, NAME as MJ_ACCORDION_TEXT};
use crate::elements::body::mj_accordion_title::{MJAccordionTitle, NAME as MJ_ACCORDION_TITLE};
use crate::parser::{Error, MJMLParser};
use crate::util::attributes::{Attributes, Merge};
use crate::util::header::Header;
use xmlparser::{StrSpan, Tokenizer};

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default();
}

struct MJAccordionElementParser<'h, 'p> {
    header: &'h Header,
    parent_attributes: &'p Attributes,
    attributes: Attributes,
    title: Option<MJAccordionTitle>,
    text: Option<MJAccordionText>,
}

impl<'h, 'p> MJAccordionElementParser<'h, 'p> {
    pub fn new(header: &'h Header, parent_attributes: &'p Attributes) -> Self {
        Self {
            header,
            parent_attributes,
            attributes: Attributes::default(),
            title: None,
            text: None,
        }
    }

    fn build_attributes(&self) -> Attributes {
        self.header
            .default_attributes
            .concat_attributes(super::NAME, &DEFAULT_ATTRIBUTES, &self.attributes)
            .concat(self.parent_attributes)
            .concat(&self.attributes)
    }
}

impl<'h, 'p> MJMLParser for MJAccordionElementParser<'h, 'p> {
    type Output = MJAccordionElement;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJAccordionElement {
            attributes: self.build_attributes(),
            context: None,
            title: self.title,
            text: self.text,
        })
    }

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, value: StrSpan<'a>) -> Result<(), Error> {
        self.attributes.set(name, value);
        Ok(())
    }

    fn parse_child_element<'a>(
        &mut self,
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<(), Error> {
        let child_attrs = super::build_children_attributes(&self.build_attributes());
        match tag.as_str() {
            MJ_ACCORDION_TITLE => {
                self.title = Some(MJAccordionTitle::parse(
                    tokenizer,
                    self.header,
                    &child_attrs,
                )?);
            }
            MJ_ACCORDION_TEXT => {
                self.text = Some(MJAccordionText::parse(
                    tokenizer,
                    self.header,
                    &child_attrs,
                )?);
            }
            _ => return Err(Error::UnexpectedElement(tag.start())),
        };
        Ok(())
    }
}

impl MJAccordionElement {
    pub fn parse<'a>(
        tokenizer: &mut Tokenizer<'a>,
        header: &Header,
        attrs: &Attributes,
    ) -> Result<MJAccordionElement, Error> {
        MJAccordionElementParser::new(header, attrs)
            .parse(tokenizer)?
            .build()
    }
}
