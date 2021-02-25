use super::children::MJHeadChild;
use super::MJHead;
use crate::elements::head::prelude::HeadComponent;
use crate::parser::{Error, MJMLParser};
use crate::util::header::Header;
use crate::Options;
use xmlparser::{StrSpan, Tokenizer};

struct MJHeadParser<'o> {
    options: &'o Options,
    children: Vec<MJHeadChild>,
}

impl<'o> MJHeadParser<'o> {
    pub fn new(options: &'o Options) -> Self {
        Self {
            options,
            children: Vec::new(),
        }
    }
}

impl<'o> MJMLParser for MJHeadParser<'o> {
    type Output = MJHead;

    fn build(self) -> Result<Self::Output, Error> {
        let mut header = Header::from(self.options);
        self.children
            .iter()
            .for_each(|item| item.update_header(&mut header));
        Ok(MJHead {
            header,
            context: None,
            children: self.children,
        })
    }

    fn parse_child_element<'a>(
        &mut self,
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<(), Error> {
        self.children.push(MJHeadChild::parse(tag, tokenizer)?);
        Ok(())
    }
}

impl<'a> MJHead {
    pub fn parse(tokenizer: &mut Tokenizer<'a>, opts: &Options) -> Result<MJHead, Error> {
        MJHeadParser::new(opts).parse(tokenizer)?.build()
    }
}
