use crate::elements::body::prelude::BodyComponent;
use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::util::attributes::Attributes;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;
use crate::util::tag::Tag;
use xmlparser::{StrSpan, Tokenizer};

use crate::elements::body::comment::Comment;
use crate::elements::body::mj_accordion::{MJAccordion, NAME as MJ_ACCORDION};
use crate::elements::body::mj_accordion_element::{
    MJAccordionElement, NAME as MJ_ACCORDION_ELEMENT,
};
use crate::elements::body::mj_button::{MJButton, NAME as MJ_BUTTON};
use crate::elements::body::mj_carousel::{MJCarousel, NAME as MJ_CAROUSEL};
use crate::elements::body::mj_carousel_image::{MJCarouselImage, NAME as MJ_CAROUSEL_IMAGE};
use crate::elements::body::mj_column::{MJColumn, NAME as MJ_COLUMN};
use crate::elements::body::mj_divider::{MJDivider, NAME as MJ_DIVIDER};
use crate::elements::body::mj_group::{MJGroup, NAME as MJ_GROUP};
use crate::elements::body::mj_hero::{MJHero, NAME as MJ_HERO};
use crate::elements::body::mj_image::{MJImage, NAME as MJ_IMAGE};
use crate::elements::body::mj_navbar::{MJNavbar, NAME as MJ_NAVBAR};
use crate::elements::body::mj_navbar_link::{MJNavbarLink, NAME as MJ_NAVBAR_LINK};
use crate::elements::body::mj_raw::{MJRaw, NAME as MJ_RAW};
use crate::elements::body::mj_section::{MJSection, NAME as MJ_SECTION};
use crate::elements::body::mj_social::{MJSocial, NAME as MJ_SOCIAL};
use crate::elements::body::mj_social_element::{MJSocialElement, NAME as MJ_SOCIAL_ELEMENT};
use crate::elements::body::mj_spacer::{MJSpacer, NAME as MJ_SPACER};
use crate::elements::body::mj_table::{MJTable, NAME as MJ_TABLE};
use crate::elements::body::mj_text::{MJText, NAME as MJ_TEXT};
use crate::elements::body::mj_wrapper::{MJWrapper, NAME as MJ_WRAPPER};
use crate::elements::body::node::Node;
use crate::elements::body::text::Text;

#[derive(Clone, Debug)]
pub enum MJBodyChild {
    Comment(Comment),
    Text(Text),
    Node(Node),
    MJAccordion(MJAccordion),
    MJAccordionElement(MJAccordionElement),
    MJButton(MJButton),
    MJCarousel(MJCarousel),
    MJCarouselImage(MJCarouselImage),
    MJColumn(MJColumn),
    MJDivider(MJDivider),
    MJGroup(MJGroup),
    MJHero(MJHero),
    MJImage(MJImage),
    MJNavbar(MJNavbar),
    MJNavbarLink(MJNavbarLink),
    MJRaw(MJRaw),
    MJSection(MJSection),
    MJSocial(MJSocial),
    MJSocialElement(MJSocialElement),
    MJSpacer(MJSpacer),
    MJTable(MJTable),
    MJText(MJText),
    MJWrapper(MJWrapper),
}

macro_rules! inner_element {
    ($root:expr) => {
        match $root {
            Self::Comment(item) => item,
            Self::Text(item) => item,
            Self::Node(item) => item,
            Self::MJAccordion(item) => item,
            Self::MJAccordionElement(item) => item,
            Self::MJButton(item) => item,
            Self::MJCarousel(item) => item,
            Self::MJCarouselImage(item) => item,
            Self::MJColumn(item) => item,
            Self::MJDivider(item) => item,
            Self::MJGroup(item) => item,
            Self::MJHero(item) => item,
            Self::MJImage(item) => item,
            Self::MJNavbar(item) => item,
            Self::MJNavbarLink(item) => item,
            Self::MJRaw(item) => item,
            Self::MJSection(item) => item,
            Self::MJSocial(item) => item,
            Self::MJSocialElement(item) => item,
            Self::MJSpacer(item) => item,
            Self::MJTable(item) => item,
            Self::MJText(item) => item,
            Self::MJWrapper(item) => item,
        }
    };
}

impl Component for MJBodyChild {
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

impl BodyComponent for MJBodyChild {
    fn get_children<'p>(&'p self) -> Box<dyn Iterator<Item = &'p MJBodyChild> + 'p> {
        self.inner().get_children()
    }

    fn get_children_len(&self) -> usize {
        self.inner().get_children_len()
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

impl MJBodyChild {
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
    ) -> Result<MJBodyChild, Error> {
        let res = match tag.as_str() {
            MJ_ACCORDION => Self::MJAccordion(MJAccordion::parse(tokenizer, header)?),
            MJ_ACCORDION_ELEMENT => Self::MJAccordionElement(MJAccordionElement::parse(
                tokenizer,
                header,
                &extra.cloned().unwrap_or_default(),
            )?),
            MJ_BUTTON => Self::MJButton(MJButton::parse(tokenizer, header)?),
            MJ_CAROUSEL => Self::MJCarousel(MJCarousel::parse(tokenizer, header)?),
            MJ_CAROUSEL_IMAGE => Self::MJCarouselImage(MJCarouselImage::parse(
                tokenizer,
                header,
                &extra.cloned().unwrap_or_default(),
            )?),
            MJ_COLUMN => Self::MJColumn(MJColumn::parse(tokenizer, header, extra)?),
            MJ_DIVIDER => Self::MJDivider(MJDivider::parse(tokenizer, header)?),
            MJ_GROUP => Self::MJGroup(MJGroup::parse(tokenizer, header)?),
            MJ_HERO => Self::MJHero(MJHero::parse(tokenizer, header)?),
            MJ_IMAGE => Self::MJImage(MJImage::parse(tokenizer, header)?),
            MJ_NAVBAR => Self::MJNavbar(MJNavbar::parse(tokenizer, header)?),
            MJ_NAVBAR_LINK => Self::MJNavbarLink(MJNavbarLink::parse(
                tokenizer,
                header,
                &extra.cloned().unwrap_or_default(),
            )?),
            MJ_RAW => Self::MJRaw(MJRaw::parse(tokenizer, header)?),
            MJ_SECTION => Self::MJSection(MJSection::parse(tokenizer, header)?),
            MJ_SOCIAL => Self::MJSocial(MJSocial::parse(tokenizer, header)?),
            MJ_SOCIAL_ELEMENT => Self::MJSocialElement(MJSocialElement::parse(
                tokenizer,
                header,
                extra.cloned().unwrap_or_default(),
            )?),
            MJ_SPACER => Self::MJSpacer(MJSpacer::parse(tokenizer, header)?),
            MJ_TABLE => Self::MJTable(MJTable::parse(tokenizer, header)?),
            MJ_TEXT => Self::MJText(MJText::parse(tokenizer, header)?),
            MJ_WRAPPER => Self::MJWrapper(MJWrapper::parse(tokenizer, header)?),
            _ => Self::Node(Node::parse(tag, tokenizer, header)?),
        };
        Ok(res)
    }

    pub fn is_raw(&self) -> bool {
        matches!(self, MJBodyChild::Node(_))
            || matches!(self, MJBodyChild::Comment(_))
            || matches!(self, MJBodyChild::Text(_))
    }
}

impl MJBodyChild {
    pub fn comment(content: String) -> Self {
        Comment::from(content).into()
    }
}

impl MJBodyChild {
    pub fn text(content: String) -> Self {
        Text::from(content).into()
    }
}

impl From<Comment> for MJBodyChild {
    fn from(elt: Comment) -> Self {
        Self::Comment(elt)
    }
}

impl From<Text> for MJBodyChild {
    fn from(elt: Text) -> Self {
        Self::Text(elt)
    }
}
