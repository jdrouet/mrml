#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use std::marker::PhantomData;

use crate::prelude::{Component, StaticTag};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(untagged))]
pub enum MjIncludeBodyChild {
    Comment(crate::comment::Comment),
    MjAccordion(crate::mj_accordion::MjAccordion),
    MjButton(crate::mj_button::MjButton),
    MjCarousel(crate::mj_carousel::MjCarousel),
    MjColumn(crate::mj_column::MjColumn),
    MjDivider(crate::mj_divider::MjDivider),
    MjGroup(crate::mj_group::MjGroup),
    MjHero(crate::mj_hero::MjHero),
    MjImage(crate::mj_image::MjImage),
    MjNavbar(crate::mj_navbar::MjNavbar),
    MjRaw(crate::mj_raw::MjRaw),
    MjSection(crate::mj_section::MjSection),
    MjSocial(crate::mj_social::MjSocial),
    MjSpacer(crate::mj_spacer::MjSpacer),
    MjTable(crate::mj_table::MjTable),
    MjText(crate::mj_text::MjText),
    MjWrapper(crate::mj_wrapper::MjWrapper),
    Node(crate::node::Node<crate::mj_body::MjBodyChild>),
    Text(crate::text::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(rename_all = "snake_case"))]
pub enum MjIncludeBodyKind {
    Mjml,
    Html,
}

impl AsRef<str> for MjIncludeBodyKind {
    fn as_ref(&self) -> &str {
        match self {
            Self::Html => "html",
            Self::Mjml => "mjml",
        }
    }
}

#[cfg(any(feature = "json", feature = "print"))]
impl MjIncludeBodyKind {
    fn is_default(&self) -> bool {
        matches!(self, Self::Mjml)
    }
}

impl Default for MjIncludeBodyKind {
    fn default() -> Self {
        Self::Mjml
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
pub struct MjIncludeBodyAttributes {
    pub path: String,
    #[cfg_attr(
        feature = "json",
        serde(
            default,
            rename = "type",
            skip_serializing_if = "MjIncludeBodyKind::is_default"
        )
    )]
    pub kind: MjIncludeBodyKind,
}

#[cfg(test)]
impl MjIncludeBodyAttributes {
    pub fn new<P: Into<String>>(path: P) -> Self {
        Self {
            path: path.into(),
            kind: Default::default(),
        }
    }

    pub fn with_kind(mut self, kind: MjIncludeBodyKind) -> Self {
        self.kind = kind;
        self
    }
}

pub struct MjIncludeBodyTag;

impl StaticTag for MjIncludeBodyTag {
    fn static_tag() -> &'static str {
        super::NAME
    }
}

pub type MjIncludeBodyInner =
    Component<PhantomData<MjIncludeBodyTag>, MjIncludeBodyAttributes, Vec<MjIncludeBodyChild>>;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(transparent))]
pub struct MjIncludeBody(pub MjIncludeBodyInner);

impl MjIncludeBody {
    #[inline]
    pub fn new(attributes: MjIncludeBodyAttributes, children: Vec<MjIncludeBodyChild>) -> Self {
        Self(MjIncludeBodyInner::new(attributes, children))
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_include::body::{MjIncludeBody, MjIncludeBodyAttributes, MjIncludeBodyKind};

    #[test]
    fn should_debug() {
        let element = MjIncludeBody::new(
            MjIncludeBodyAttributes::new("partial.mjml").with_kind(MjIncludeBodyKind::Mjml),
            Default::default(),
        );
        let _ = format!("{element:?}");
    }
}
