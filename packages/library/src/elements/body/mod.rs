use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::util::attributes::Attributes;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;
use crate::util::tag::Tag;
use prelude::BodyComponent;
use xmlparser::{StrSpan, Tokenizer};

pub mod comment;
pub mod mj_accordion;
pub mod mj_accordion_element;
pub mod mj_accordion_text;
pub mod mj_accordion_title;
pub mod mj_body;
pub mod mj_button;
pub mod mj_carousel;
pub mod mj_carousel_image;
pub mod mj_column;
pub mod mj_divider;
pub mod mj_group;
pub mod mj_hero;
pub mod mj_image;
pub mod mj_navbar;
pub mod mj_navbar_link;
pub mod mj_raw;
pub mod mj_section;
pub mod mj_social;
pub mod mj_social_element;
pub mod mj_spacer;
pub mod mj_table;
pub mod mj_text;
pub mod mj_wrapper;
pub mod node;
pub mod prelude;
pub mod raw;
pub mod text;

#[derive(Clone, Debug)]
pub enum BodyElement {
    Comment(comment::Comment),
    Text(text::Text),
    Node(node::Node),
    MJAccordion(mj_accordion::MJAccordion),
    MJAccordionElement(mj_accordion_element::MJAccordionElement),
    MJButton(mj_button::MJButton),
    MJCarousel(mj_carousel::MJCarousel),
    MJCarouselImage(mj_carousel_image::MJCarouselImage),
    MJColumn(mj_column::MJColumn),
    MJDivider(mj_divider::MJDivider),
    MJGroup(mj_group::MJGroup),
    MJHero(mj_hero::MJHero),
    MJImage(mj_image::MJImage),
    MJNavbar(mj_navbar::MJNavbar),
    MJNavbarLink(mj_navbar_link::MJNavbarLink),
    MJRaw(mj_raw::MJRaw),
    MJSection(mj_section::MJSection),
    MJSocial(mj_social::MJSocial),
    MJSocialElement(mj_social_element::MJSocialElement),
    MJSpacer(mj_spacer::MJSpacer),
    MJTable(mj_table::MJTable),
    MJText(mj_text::MJText),
    MJWrapper(mj_wrapper::MJWrapper),
}

macro_rules! inner_element {
    ($root:expr) => {
        match $root {
            BodyElement::Comment(item) => item,
            BodyElement::Text(item) => item,
            BodyElement::Node(item) => item,
            BodyElement::MJAccordion(item) => item,
            BodyElement::MJAccordionElement(item) => item,
            BodyElement::MJButton(item) => item,
            BodyElement::MJCarousel(item) => item,
            BodyElement::MJCarouselImage(item) => item,
            BodyElement::MJColumn(item) => item,
            BodyElement::MJDivider(item) => item,
            BodyElement::MJGroup(item) => item,
            BodyElement::MJHero(item) => item,
            BodyElement::MJImage(item) => item,
            BodyElement::MJNavbar(item) => item,
            BodyElement::MJNavbarLink(item) => item,
            BodyElement::MJRaw(item) => item,
            BodyElement::MJSection(item) => item,
            BodyElement::MJSocial(item) => item,
            BodyElement::MJSocialElement(item) => item,
            BodyElement::MJSpacer(item) => item,
            BodyElement::MJTable(item) => item,
            BodyElement::MJText(item) => item,
            BodyElement::MJWrapper(item) => item,
        }
    };
}

impl Component for BodyElement {
    fn update_header(&self, header: &mut Header) {
        self.inner().update_header(header)
    }

    fn context(&self) -> Option<&Context> {
        self.inner().context()
    }

    fn set_context(&mut self, ctx: Context) {
        self.inner_mut().set_context(ctx)
    }

    fn render(&self, header: &Header) -> Result<String, Error> {
        self.inner().render(header)
    }
}

impl BodyComponent for BodyElement {
    fn get_children(&self) -> &Vec<BodyElement> {
        self.inner().get_children()
    }
    fn get_current_width(&self) -> Option<Size> {
        self.inner().get_current_width()
    }
    fn attributes(&self) -> Option<&Attributes> {
        self.inner().attributes()
    }
    fn set_style(&self, key: &str, tag: Tag) -> Tag {
        self.inner().set_style(key, tag)
    }

    fn get_width(&self) -> Option<Size> {
        self.inner().get_width()
    }
}

impl BodyElement {
    pub fn inner_mut(&mut self) -> &mut dyn BodyComponent {
        inner_element!(self)
    }

    pub fn inner(&self) -> &dyn BodyComponent {
        inner_element!(self)
    }

    pub fn parse<'a>(
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
        header: &Header,
        extra: Option<&Attributes>,
    ) -> Result<BodyElement, Error> {
        let res = match tag.as_str() {
            mj_accordion::NAME => {
                BodyElement::MJAccordion(mj_accordion::MJAccordion::parse(tokenizer, header)?)
            }
            mj_button::NAME => {
                BodyElement::MJButton(mj_button::MJButton::parse(tokenizer, header)?)
            }
            mj_carousel::NAME => {
                BodyElement::MJCarousel(mj_carousel::MJCarousel::parse(tokenizer, header)?)
            }
            mj_column::NAME => {
                BodyElement::MJColumn(mj_column::MJColumn::parse(tokenizer, header, extra)?)
            }
            mj_divider::NAME => {
                BodyElement::MJDivider(mj_divider::MJDivider::parse(tokenizer, header)?)
            }
            mj_group::NAME => BodyElement::MJGroup(mj_group::MJGroup::parse(tokenizer, header)?),
            mj_hero::NAME => BodyElement::MJHero(mj_hero::MJHero::parse(tokenizer, header)?),
            mj_image::NAME => BodyElement::MJImage(mj_image::MJImage::parse(tokenizer, header)?),
            mj_navbar::NAME => {
                BodyElement::MJNavbar(mj_navbar::MJNavbar::parse(tokenizer, header)?)
            }
            mj_raw::NAME => BodyElement::MJRaw(mj_raw::MJRaw::parse(tokenizer, header)?),
            mj_section::NAME => {
                BodyElement::MJSection(mj_section::MJSection::parse(tokenizer, header)?)
            }
            mj_social::NAME => {
                BodyElement::MJSocial(mj_social::MJSocial::parse(tokenizer, header)?)
            }
            mj_spacer::NAME => {
                BodyElement::MJSpacer(mj_spacer::MJSpacer::parse(tokenizer, header)?)
            }
            mj_table::NAME => BodyElement::MJTable(mj_table::MJTable::parse(tokenizer, header)?),
            mj_text::NAME => BodyElement::MJText(mj_text::MJText::parse(tokenizer, header)?),
            mj_wrapper::NAME => {
                BodyElement::MJWrapper(mj_wrapper::MJWrapper::parse(tokenizer, header)?)
            }
            _ => BodyElement::Node(node::Node::parse(tag, tokenizer, header)?),
        };
        Ok(res)
    }

    pub fn is_raw(&self) -> bool {
        matches!(self, BodyElement::Node(_))
            || matches!(self, BodyElement::Comment(_))
            || matches!(self, BodyElement::Text(_))
    }
}

impl BodyElement {
    pub fn comment(content: String) -> Self {
        comment::Comment::from(content).into()
    }
}

impl BodyElement {
    pub fn text(content: String) -> Self {
        text::Text::from(content).into()
    }
}

impl From<comment::Comment> for BodyElement {
    fn from(elt: comment::Comment) -> Self {
        Self::Comment(elt)
    }
}

impl From<text::Text> for BodyElement {
    fn from(elt: text::Text) -> Self {
        Self::Text(elt)
    }
}
