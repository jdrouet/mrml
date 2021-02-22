use super::MJHero;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::parser::MJMLParser;
use crate::util::attributes::*;
use crate::util::header::Header;
use xmlparser::{StrSpan, Tokenizer};

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("background-color", "#ffffff")
        .add("background-position", "center center")
        .add("height", "0px")
        .add("mode", "fixed-height")
        .add("padding", "0px")
        .add("vertical-align", "top");
}

struct MJHeroParser<'h> {
    header: &'h Header,
    attributes: Attributes,
    children: Vec<BodyElement>,
}

impl<'h> MJHeroParser<'h> {
    pub fn new(header: &'h Header) -> Self {
        Self {
            header,
            attributes: Attributes::new(),
            children: Vec::new(),
        }
    }
}

impl<'h> MJMLParser for MJHeroParser<'h> {
    type Output = MJHero;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJHero {
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
        self.children.push(BodyElement::comment(value.to_string()));
        Ok(())
    }

    fn parse_child_text(&mut self, value: StrSpan) -> Result<(), Error> {
        self.children.push(BodyElement::text(value.to_string()));
        Ok(())
    }

    fn parse_child_element<'a>(
        &mut self,
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<(), Error> {
        self.children
            .push(BodyElement::parse(tag, tokenizer, self.header, None)?);
        Ok(())
    }
}

impl MJHero {
    pub fn parse<'a>(tokenizer: &mut Tokenizer<'a>, header: &Header) -> Result<MJHero, Error> {
        MJHeroParser::new(header).parse(tokenizer)?.build()
    }
}
