use super::children::MJBodyChild;
use super::MJBody;
use crate::elements::Error;
use crate::parser::MJMLParser;
use crate::util::attributes::*;
use crate::util::header::Header;
use xmlparser::{StrSpan, Tokenizer};

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default().add("width", "600px");
}

struct MJBodyParser<'h> {
    header: &'h Header,
    attributes: Attributes,
    children: Vec<MJBodyChild>,
}

impl<'h> MJBodyParser<'h> {
    fn new(header: &'h Header) -> Self {
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
}

impl<'h> MJMLParser for MJBodyParser<'h> {
    type Output = MJBody;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJBody {
            attributes: self.build_attributes(),
            context: None,
            children: self.children,
            exists: true,
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
        self.children
            .push(MJBodyChild::parse(tag, tokenizer, self.header, None)?);
        Ok(())
    }
}

impl MJBody {
    pub fn parse<'a>(tokenizer: &mut Tokenizer<'a>, header: &Header) -> Result<MJBody, Error> {
        MJBodyParser::new(header).parse(tokenizer)?.build()
    }
}
