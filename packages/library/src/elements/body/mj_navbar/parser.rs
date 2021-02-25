use super::MJNavbar;
use crate::elements::body::mj_body::children::MJBodyChild;
use crate::elements::body::mj_navbar_link::{MJNavbarLink, NAME as MJ_NAVBAR_LINK};
use crate::elements::error::Error;
use crate::parser::MJMLParser;
use crate::util::attributes::*;
use crate::util::header::Header;
use crate::util::id::Generator as IdGenerator;
use xmlparser::{StrSpan, Tokenizer};

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("align", "center")
        .add("ico-align", "center")
        .add("ico-open", "&#9776;")
        .add("ico-close", "&#8855;")
        .add("ico-color", "#000000")
        .add("ico-font-family", "Ubuntu, Helvetica, Arial, sans-serif")
        .add("ico-font-size", "30px")
        .add("ico-text-transform", "uppercase")
        .add("ico-padding", "10px")
        .add("ico-text-decoration", "none")
        .add("ico-line-height", "30px");
}

struct MJNavbarParser<'h> {
    header: &'h Header,
    attributes: Attributes,
    children: Vec<MJBodyChild>,
    id: String,
}

impl<'h> MJNavbarParser<'h> {
    pub fn new(header: &'h Header) -> Self {
        let generator: IdGenerator = header.id_generator;
        Self {
            header,
            attributes: Attributes::default(),
            children: Vec::new(),
            id: generator(8),
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

impl<'h> MJMLParser for MJNavbarParser<'h> {
    type Output = MJNavbar;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJNavbar {
            attributes: self.build_attributes(),
            context: None,
            children: self.children,
            id: self.id,
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
        if tag.as_str() != MJ_NAVBAR_LINK {
            return Err(Error::UnexpectedElement(tag.to_string()));
        }
        let child_attrs = self.get_children_attributes();
        let element = MJNavbarLink::parse(tokenizer, self.header, &child_attrs)?;
        self.children.push(MJBodyChild::MJNavbarLink(element));
        Ok(())
    }
}

impl MJNavbar {
    pub fn parse<'a>(tokenizer: &mut Tokenizer<'a>, header: &Header) -> Result<MJNavbar, Error> {
        MJNavbarParser::new(header).parse(tokenizer)?.build()
    }
}