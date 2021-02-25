use super::MJNavbarLink;
use crate::parser::{Error, MJMLParser};
use crate::util::attributes::{Attributes, Merge};
use crate::util::header::Header;
use xmlparser::{StrSpan, Tokenizer};

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("color", "#000000")
        .add("font-family", "Ubuntu, Helvetica, Arial, sans-serif")
        .add("font-size", "13px")
        .add("font-weight", "normal")
        .add("line-height", "22px")
        .add("padding", "15px 10px")
        .add("target", "_blank")
        .add("text-decoration", "none")
        .add("text-transform", "uppercase");
}

struct MJNavbarLinkParser<'h> {
    header: &'h Header,
    extra: Attributes,
    attributes: Attributes,
    content: Option<String>,
}

impl<'h> MJNavbarLinkParser<'h> {
    pub fn new(header: &'h Header, extra: Attributes) -> Self {
        Self {
            header,
            extra,
            attributes: Attributes::new(),
            content: None,
        }
    }
}

impl<'h> MJMLParser for MJNavbarLinkParser<'h> {
    type Output = MJNavbarLink;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJNavbarLink {
            attributes: self
                .header
                .default_attributes
                .concat_attributes(super::NAME, &DEFAULT_ATTRIBUTES, &self.attributes)
                .concat(&self.extra)
                .concat(&self.attributes),
            context: None,
            content: self.content,
        })
    }

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, value: StrSpan<'a>) -> Result<(), Error> {
        self.attributes.set(name, value);
        Ok(())
    }

    fn parse_child_text(&mut self, value: StrSpan) -> Result<(), Error> {
        self.content = Some(value.to_string());
        Ok(())
    }
}

impl MJNavbarLink {
    pub fn parse<'a>(
        tokenizer: &mut Tokenizer<'a>,
        header: &Header,
        extra: &Attributes,
    ) -> Result<MJNavbarLink, Error> {
        let mut parent = extra.clone();
        if parent.get("text-padding").is_none() {
            parent.set("text-padding", "4px 4px 4px 0");
        }
        MJNavbarLinkParser::new(header, parent)
            .parse(tokenizer)?
            .build()
    }

    pub fn parse_link<'a>(
        tokenizer: &mut Tokenizer<'a>,
        header: &Header,
        extra: &Attributes,
    ) -> Result<MJNavbarLink, Error> {
        MJNavbarLinkParser::new(header, extra.clone())
            .parse(tokenizer)?
            .build()
    }
}
