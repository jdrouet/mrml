use super::MJTable;
use crate::elements::body::mj_body::children::MJBodyChild;
use crate::elements::body::raw::RawElement;
use crate::elements::error::Error;
use crate::parser::MJMLParser;
use crate::util::attributes::*;
use crate::util::header::Header;
use xmlparser::{StrSpan, Tokenizer};

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
    children: Vec<MJBodyChild>,
}

impl<'h> MJTableParser<'h> {
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
        self.children
            .push(RawElement::conditional_parse(tag, tokenizer, self.header, true)?.into());
        Ok(())
    }
}

impl MJTable {
    pub fn parse<'a>(tokenizer: &mut Tokenizer<'a>, header: &Header) -> Result<MJTable, Error> {
        MJTableParser::new(header).parse(tokenizer)?.build()
    }
}
