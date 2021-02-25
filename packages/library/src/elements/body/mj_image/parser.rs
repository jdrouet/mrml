use super::MJImage;
use crate::parser::{Error, MJMLParser};
use crate::util::attributes::{Attributes, Merge};
use crate::util::header::Header;
use xmlparser::{StrSpan, Tokenizer};

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("align", "center")
        .add("border", "0")
        .add("height", "auto")
        .add("padding", "10px 25px")
        .add("target", "_blank")
        .add("font-size", "13px");
}

struct MJImageParser<'h> {
    header: &'h Header,
    attributes: Attributes,
}

impl<'h> MJImageParser<'h> {
    pub fn new(header: &'h Header) -> Self {
        Self {
            header,
            attributes: Attributes::default(),
        }
    }
}

impl<'h> MJMLParser for MJImageParser<'h> {
    type Output = MJImage;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJImage {
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

impl MJImage {
    pub fn parse<'a>(tokenizer: &mut Tokenizer<'a>, header: &Header) -> Result<MJImage, Error> {
        MJImageParser::new(header).parse(tokenizer)?.build()
    }
}
