use super::MJAccordionTitle;
use crate::elements::error::Error;
use crate::parser::MJMLParser;
use crate::util::attributes::*;
use crate::util::header::Header;
use xmlparser::{StrSpan, Tokenizer};

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
}

impl<'h, 'p> MJMLParser for MJAccordionTitleParser<'h, 'p> {
    type Output = MJAccordionTitle;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJAccordionTitle {
            attributes: self
                .header
                .default_attributes
                .concat_attributes(super::NAME, &DEFAULT_ATTRIBUTES, &self.attributes)
                .concat(self.parent_attributes)
                .concat(&self.attributes),
            context: None,
            content: self.content,
        })
    }

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, value: StrSpan<'a>) -> Result<(), Error> {
        self.attributes.set(name, value);
        Ok(())
    }

    fn parse_child_text(&mut self, value: StrSpan) -> Result<(), Error> {
        self.content.push_str(value.as_str());
        Ok(())
    }
}

impl MJAccordionTitle {
    pub fn parse<'a>(
        tokenizer: &mut Tokenizer<'a>,
        header: &Header,
        attrs: &Attributes,
    ) -> Result<MJAccordionTitle, Error> {
        MJAccordionTitleParser::new(header, attrs)
            .parse(tokenizer)?
            .build()
    }
}
