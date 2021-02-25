use super::MJDivider;
use crate::parser::{Error, MJMLParser};
use crate::util::attributes::{Attributes, Merge};
use crate::util::header::Header;
use xmlparser::{StrSpan, Tokenizer};

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("border-color", "#000000")
        .add("border-style", "solid")
        .add("border-width", "4px")
        .add("padding", "10px 25px")
        .add("width", "100%");
}

struct MJDividerParser<'h> {
    header: &'h Header,
    attributes: Attributes,
}

impl<'h> MJDividerParser<'h> {
    pub fn new(header: &'h Header) -> Self {
        Self {
            header,
            attributes: Attributes::new(),
        }
    }
}

impl<'h> MJMLParser for MJDividerParser<'h> {
    type Output = MJDivider;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJDivider {
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

impl MJDivider {
    pub fn parse<'a>(tokenizer: &mut Tokenizer<'a>, header: &Header) -> Result<MJDivider, Error> {
        MJDividerParser::new(header).parse(tokenizer)?.build()
    }
}
