use super::{MJSection, DEFAULT_BACKGROUND_POSITION};
use crate::elements::body::mj_body::children::MJBodyChild;
use crate::parser::{Error, MJMLParser};
use crate::util::attributes::{Attributes, Merge};
use crate::util::header::Header;
use xmlparser::{StrSpan, Tokenizer};

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("background-position", DEFAULT_BACKGROUND_POSITION)
        .add("background-repeat", "repeat")
        .add("background-size", "auto")
        .add("direction", "ltr")
        .add("padding", "20px 0")
        .add("text-align", "center")
        .add("text-padding", "4px 4px 4px 0");
}

struct MJSectionParser<'h> {
    header: &'h Header,
    attributes: Attributes,
    children: Vec<MJBodyChild>,
}

impl<'h> MJSectionParser<'h> {
    pub fn new(header: &'h Header) -> Self {
        Self {
            header,
            attributes: Attributes::new(),
            children: vec![],
        }
    }
}

impl<'h> MJMLParser for MJSectionParser<'h> {
    type Output = MJSection;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJSection {
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
            .push(MJBodyChild::parse(tag, tokenizer, self.header, None)?);
        Ok(())
    }
}

impl MJSection {
    pub fn parse<'a>(tokenizer: &mut Tokenizer<'a>, header: &Header) -> Result<MJSection, Error> {
        MJSectionParser::new(header).parse(tokenizer)?.build()
    }
}
