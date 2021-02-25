use super::MJSpacer;
use crate::parser::{Error, MJMLParser};
use crate::util::attributes::{Attributes, Merge};
use crate::util::header::Header;
use xmlparser::{StrSpan, Tokenizer};

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default().add("height", "20px");
}

struct MJSpacerParser<'h> {
    header: &'h Header,
    attributes: Attributes,
}

impl<'h> MJSpacerParser<'h> {
    pub fn new(header: &'h Header) -> Self {
        Self {
            header,
            attributes: Attributes::new(),
        }
    }
}

impl<'h> MJMLParser for MJSpacerParser<'h> {
    type Output = MJSpacer;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJSpacer {
            attributes: self
                .header
                .default_attributes
                .concat_attributes(super::NAME, &DEFAULT_ATTRIBUTES, &self.attributes)
                .concat(&self.attributes),
            context: None,
        })
    }

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, value: StrSpan<'a>) -> Result<(), Error> {
        self.attributes.set(name, value);
        Ok(())
    }
}

impl MJSpacer {
    pub fn parse<'a>(tokenizer: &mut Tokenizer<'a>, header: &Header) -> Result<MJSpacer, Error> {
        MJSpacerParser::new(header).parse(tokenizer)?.build()
    }
}
