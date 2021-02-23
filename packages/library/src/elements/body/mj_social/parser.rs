use super::MJSocial;
use crate::elements::body::mj_body::children::MJBodyChild;
use crate::elements::body::mj_social_element::{MJSocialElement, NAME as MJ_SOCIAL_ELEMENT};
use crate::elements::error::Error;
use crate::parser::MJMLParser;
use crate::util::attributes::*;
use crate::util::header::Header;
use xmlparser::{StrSpan, Tokenizer};

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("align", "center")
        .add("border-radius", "3px")
        .add("color", "#333333")
        .add("font-family", "Ubuntu, Helvetica, Arial, sans-serif")
        .add("font-size", "13px")
        .add("icon-size", "20px")
        .add("line-height", "22px")
        .add("mode", "horizontal")
        .add("padding", "10px 25px")
        .add("text-decoration", "none");
}

struct MJSocialParser<'h> {
    header: &'h Header,
    attributes: Attributes,
    children: Vec<MJBodyChild>,
}

impl<'h> MJSocialParser<'h> {
    pub fn new(header: &'h Header) -> Self {
        Self {
            header,
            attributes: Attributes::default(),
            children: Vec::new(),
        }
    }

    fn build_attributes(&self) -> Attributes {
        self.header
            .default_attributes
            .concat_attributes(super::NAME, &DEFAULT_ATTRIBUTES, &self.attributes)
            .concat(&self.attributes)
    }

    fn get_children_attributes(&self) -> Attributes {
        super::build_children_attributes(&self.build_attributes())
    }
}

impl<'h> MJMLParser for MJSocialParser<'h> {
    type Output = MJSocial;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJSocial {
            attributes: self.build_attributes(),
            context: None,
            children: self.children,
        })
    }

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, value: StrSpan<'a>) -> Result<(), Error> {
        self.attributes.set(name, value);
        Ok(())
    }

    fn parse_child_element<'a>(
        &mut self,
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<(), Error> {
        if tag.as_str() != MJ_SOCIAL_ELEMENT {
            return Err(Error::UnexpectedElement(tag.to_string()));
        }
        let child_attrs = self.get_children_attributes();
        let element = MJSocialElement::parse(tokenizer, self.header, child_attrs)?;
        self.children.push(MJBodyChild::MJSocialElement(element));
        Ok(())
    }
}

impl MJSocial {
    pub fn parse<'a>(tokenizer: &mut Tokenizer<'a>, header: &Header) -> Result<MJSocial, Error> {
        MJSocialParser::new(header).parse(tokenizer)?.build()
    }
}
