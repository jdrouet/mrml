use super::MJCarousel;
use crate::elements::body::mj_carousel_image::{MJCarouselImage, NAME as MJ_CAROUSEL_IMAGE};
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::parser::MJMLParser;
use crate::util::attributes::*;
use crate::util::header::Header;
use crate::util::id::Generator as IdGenerator;
use xmlparser::{StrSpan, Tokenizer};

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("align", "center")
        .add("border-radius", "6px")
        .add("icon-width", "44px")
        .add("left-icon", "https://i.imgur.com/xTh3hln.png")
        .add("right-icon", "https://i.imgur.com/os7o9kz.png")
        .add("thumbnails", "visible")
        .add("tb-border", "2px solid transparent")
        .add("tb-border-radius", "6px")
        .add("tb-hover-border-color", "#fead0d")
        .add("tb-selected-border-color", "#cccccc");
}

struct MJCarouselParser<'h> {
    header: &'h Header,
    id: String,
    attributes: Attributes,
    children: Vec<BodyElement>,
}

impl<'h> MJCarouselParser<'h> {
    pub fn new(header: &'h Header) -> Self {
        let generator: IdGenerator = header.id_generator;
        Self {
            header,
            id: generator(8),
            attributes: Attributes::default(),
            children: Vec::default(),
        }
    }

    fn build_attributes(&self) -> Attributes {
        self.header
            .default_attributes
            .concat_attributes(super::NAME, &DEFAULT_ATTRIBUTES, &self.attributes)
            .concat(&self.attributes)
    }

    fn get_children_attributes(&self) -> Attributes {
        super::build_children_attributes(self.id.as_str(), &self.build_attributes())
    }
}

impl<'h> MJMLParser for MJCarouselParser<'h> {
    type Output = MJCarousel;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJCarousel {
            attributes: self
                .header
                .default_attributes
                .concat_attributes(super::NAME, &DEFAULT_ATTRIBUTES, &self.attributes)
                .concat(&self.attributes),
            id: self.id,
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

    fn parse_child_element<'a>(
        &mut self,
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<(), Error> {
        if tag.as_str() != MJ_CAROUSEL_IMAGE {
            return Err(Error::UnexpectedElement(tag.to_string()));
        }
        let child_attrs = self.get_children_attributes();
        let element = MJCarouselImage::parse(tokenizer, self.header, &child_attrs)?;
        self.children.push(BodyElement::MJCarouselImage(element));
        Ok(())
    }
}

impl MJCarousel {
    pub fn parse<'a>(tokenizer: &mut Tokenizer<'a>, header: &Header) -> Result<MJCarousel, Error> {
        MJCarouselParser::new(header).parse(tokenizer)?.build()
    }
}
