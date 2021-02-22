use super::MJButton;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::parser::MJMLParser;
use crate::util::attributes::*;
use crate::util::header::Header;
use xmlparser::{StrSpan, Tokenizer};

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("align", "center")
        .add("background-color", "#414141")
        .add("border", "none")
        .add("border-radius", "3px")
        .add("color", "#ffffff")
        .add("font-family", "Ubuntu, Helvetica, Arial, sans-serif")
        .add("font-size", "13px")
        .add("font-weight", "normal")
        .add("inner-padding", "10px 25px")
        .add("line-height", "120%")
        .add("padding", "10px 25px")
        .add("target", "_blank")
        .add("text-decoration", "none")
        .add("text-transform", "none")
        .add("vertical-align", "middle");
}

struct MJButtonParser<'h> {
    header: &'h Header,
    attributes: Attributes,
    children: Vec<BodyElement>,
}

impl<'h> MJButtonParser<'h> {
    pub fn new(header: &'h Header) -> Self {
        Self {
            header,
            attributes: Attributes::new(),
            children: vec![],
        }
    }
}

impl<'h> MJMLParser for MJButtonParser<'h> {
    type Output = MJButton;

    fn build(self) -> Result<Self::Output, Error> {
        let attributes = self
            .header
            .default_attributes
            .concat_attributes(super::NAME, &DEFAULT_ATTRIBUTES, &self.attributes)
            .concat(&self.attributes);
        Ok(MJButton {
            attributes,
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

impl MJButton {
    pub fn parse(tokenizer: &mut Tokenizer, header: &Header) -> Result<MJButton, Error> {
        MJButtonParser::new(header).parse(tokenizer)?.build()
    }
}
