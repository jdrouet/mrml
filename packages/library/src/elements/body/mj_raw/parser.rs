use super::MJRaw;
use crate::elements::body::mj_body::children::MJBodyChild;
use crate::elements::body::raw::RawElement;
use crate::parser::{Error, MJMLParser};
use crate::util::header::Header;
use xmlparser::{StrSpan, Tokenizer};

struct MJRawParser<'h> {
    header: &'h Header,
    children: Vec<MJBodyChild>,
}

impl<'h> MJRawParser<'h> {
    pub fn new(header: &'h Header) -> Self {
        Self {
            header,
            children: vec![],
        }
    }
}

impl<'h> MJMLParser for MJRawParser<'h> {
    type Output = MJRaw;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJRaw {
            context: None,
            children: self.children,
        })
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

impl MJRaw {
    pub fn parse<'a>(tokenizer: &mut Tokenizer<'a>, header: &Header) -> Result<MJRaw, Error> {
        MJRawParser::new(header).parse(tokenizer)?.build()
    }
}
