use super::MJCarouselImage;
use crate::elements::error::Error;
use crate::parser::MJMLParser;
use crate::util::attributes::*;
use crate::util::header::Header;
use xmlparser::{StrSpan, Tokenizer};

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default().add("target", "_blank");
}

struct MJCarouselImageParser<'h> {
    header: &'h Header,
    parent_attributes: Attributes,
    carousel_id: String,
    attributes: Attributes,
    content: Option<String>,
}

impl<'h> MJCarouselImageParser<'h> {
    pub fn new(header: &'h Header, mut extra: Attributes) -> Self {
        if extra.get("text-padding").is_none() {
            extra.set("text-padding", "4px 4px 4px 0");
        }
        let carousel_id = extra.get("carousel-id").cloned().unwrap_or_default();
        Self {
            header,
            parent_attributes: extra,
            carousel_id,
            attributes: Attributes::default(),
            content: None,
        }
    }
}

impl<'h> MJMLParser for MJCarouselImageParser<'h> {
    type Output = MJCarouselImage;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJCarouselImage {
            attributes: self
                .header
                .default_attributes
                .concat_attributes(super::NAME, &DEFAULT_ATTRIBUTES, &self.attributes)
                .concat(&self.parent_attributes)
                .concat(&self.attributes),
            carousel_id: self.carousel_id,
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

impl MJCarouselImage {
    pub fn parse<'a>(
        tokenizer: &mut Tokenizer<'a>,
        header: &Header,
        extra: &Attributes,
    ) -> Result<MJCarouselImage, Error> {
        MJCarouselImageParser::new(header, extra.clone())
            .parse(tokenizer)?
            .build()
    }
}
