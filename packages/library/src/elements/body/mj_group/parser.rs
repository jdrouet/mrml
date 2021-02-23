use super::MJGroup;
use crate::elements::body::mj_body::children::MJBodyChild;
use crate::elements::error::Error;
use crate::parser::MJMLParser;
use crate::util::attributes::*;
use crate::util::header::Header;
use xmlparser::{StrSpan, Tokenizer};

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default().add("direction", "ltr");
}

struct MJGroupParser<'h> {
    header: &'h Header,
    attributes: Attributes,
    child_attributes: Attributes,
    children: Vec<MJBodyChild>,
}

impl<'h> MJGroupParser<'h> {
    pub fn new(header: &'h Header) -> Self {
        Self {
            header,
            attributes: Attributes::default(),
            child_attributes: Attributes::default().add("mobile-width", "mobile-width"),
            children: Vec::new(),
        }
    }
}

impl<'h> MJMLParser for MJGroupParser<'h> {
    type Output = MJGroup;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJGroup {
            attributes: self
                .header
                .default_attributes
                .concat_attributes(super::NAME, &DEFAULT_ATTRIBUTES, &self.attributes)
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
        self.children.push(MJBodyChild::comment(value.to_string()));
        Ok(())
    }

    fn parse_child_text(&mut self, value: StrSpan) -> Result<(), Error> {
        self.children.push(MJBodyChild::text(value.to_string()));
        Ok(())
    }

    fn parse_child_element<'a>(
        &mut self,
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<(), Error> {
        self.children.push(MJBodyChild::parse(
            tag,
            tokenizer,
            self.header,
            Some(&self.child_attributes),
        )?);
        Ok(())
    }
}

impl MJGroup {
    pub fn parse<'a>(tokenizer: &mut Tokenizer<'a>, header: &Header) -> Result<MJGroup, Error> {
        MJGroupParser::new(header).parse(tokenizer)?.build()
    }
}
