use super::MJAccordionText;
use crate::elements::body::raw::RawElement;
use crate::elements::error::Error;
use crate::parser::MJMLParser;
use crate::util::attributes::*;
use crate::util::header::Header;
use xmlparser::{StrSpan, Tokenizer};

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
}

impl<'h, 'p> MJMLParser for MJAccordionTextParser<'h, 'p> {
    type Output = MJAccordionText;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJAccordionText {
            attributes: self
                .header
                .default_attributes
                .concat_attributes(super::NAME, &DEFAULT_ATTRIBUTES, &self.attributes)
                .concat(self.parent_attributes)
                .concat(&self.attributes),
            context: None,
            children: self.children,
        })
    }

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, value: StrSpan<'a>) -> Result<(), Error> {
        self.attributes.set(name, value);
        Ok(())
    }

    fn parse_child_comment(&mut self, value: StrSpan) -> Result<(), Error> {
        self.children.push(RawElement::comment(value.to_string()));
        Ok(())
    }

    fn parse_child_text(&mut self, value: StrSpan) -> Result<(), Error> {
        self.children.push(RawElement::text(value.to_string()));
        Ok(())
    }

    fn parse_child_element<'a>(
        &mut self,
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<(), Error> {
        self.children.push(RawElement::conditional_parse(
            tag,
            tokenizer,
            self.header,
            true,
        )?);
        Ok(())
    }
}

impl MJAccordionText {
    pub fn parse(
        tokenizer: &mut Tokenizer,
        header: &Header,
        attrs: &Attributes,
    ) -> Result<MJAccordionText, Error> {
        MJAccordionTextParser::new(header, attrs)
            .parse(tokenizer)?
            .build()
    }
}
