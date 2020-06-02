use crate::util::{Context, Header, Style};
use roxmltree::Node;
use std::collections::HashMap;

pub mod mj_body;
pub mod mj_button;
pub mod mj_column;
pub mod mj_divider;
pub mod mj_hero;
pub mod mj_image;
pub mod mj_section;
pub mod mj_social;
pub mod mj_social_element;
pub mod mj_text;
pub mod prelude;
pub mod raw;

use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::Options;
use prelude::BodyComponent;

#[derive(Clone, Debug)]
pub enum BodyElement {
    MJButton(mj_button::MJButton),
    MJColumn(mj_column::MJColumn),
    MJDivider(mj_divider::MJDivider),
    MJHero(mj_hero::MJHero),
    MJImage(mj_image::MJImage),
    MJSection(mj_section::MJSection),
    MJSocial(mj_social::MJSocial),
    MJSocialElement(mj_social_element::MJSocialElement),
    MJText(mj_text::MJText),
    Raw(raw::RawElement),
}

macro_rules! apply_fn {
    ($root:expr, $func:ident($($args:tt)*)) => {
        match $root {
            BodyElement::MJButton(item) => item.$func($($args)*),
            BodyElement::MJColumn(item) => item.$func($($args)*),
            BodyElement::MJDivider(item) => item.$func($($args)*),
            BodyElement::MJHero(item) => item.$func($($args)*),
            BodyElement::MJImage(item) => item.$func($($args)*),
            BodyElement::MJSection(item) => item.$func($($args)*),
            BodyElement::MJSocial(item) => item.$func($($args)*),
            BodyElement::MJSocialElement(item) => item.$func($($args)*),
            BodyElement::MJText(item) => item.$func($($args)*),
            BodyElement::Raw(item) => item.$func($($args)*),
        }
    };
}

impl Component for BodyElement {
    fn update_header(&self, header: &mut Header) {
        apply_fn!(self, update_header(header))
    }

    fn context(&self) -> Option<&Context> {
        apply_fn!(self, context())
    }

    fn set_context(&mut self, ctx: Context) {
        apply_fn!(self, set_context(ctx))
    }

    fn render(&self, header: &Header) -> Result<String, Error> {
        apply_fn!(self, render(header))
    }
}

impl ComponentWithAttributes for BodyElement {
    fn source_attributes(&self) -> Option<&HashMap<String, String>> {
        apply_fn!(self, source_attributes())
    }

    fn get_attribute(&self, key: &str) -> Option<String> {
        apply_fn!(self, get_attribute(key))
    }
}

impl BodyComponent for BodyElement {
    fn get_style(&self, key: &str) -> Style {
        apply_fn!(self, get_style(key))
    }
}

impl BodyElement {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>, opts: &Options) -> Result<BodyElement, Error> {
        let res = match node.tag_name().name() {
            "mj-button" => BodyElement::MJButton(mj_button::MJButton::parse(node, opts)?),
            "mj-column" => BodyElement::MJColumn(mj_column::MJColumn::parse(node, opts)?),
            "mj-divider" => BodyElement::MJDivider(mj_divider::MJDivider::parse(node, opts)?),
            "mj-hero" => BodyElement::MJHero(mj_hero::MJHero::parse(node, opts)?),
            "mj-image" => BodyElement::MJImage(mj_image::MJImage::parse(node, opts)?),
            "mj-section" => BodyElement::MJSection(mj_section::MJSection::parse(node, opts)?),
            "mj-social" => BodyElement::MJSocial(mj_social::MJSocial::parse(node, opts)?),
            "mj-social-element" => {
                BodyElement::MJSocialElement(mj_social_element::MJSocialElement::parse(node, opts)?)
            }
            "mj-text" => BodyElement::MJText(mj_text::MJText::parse(node, opts)?),
            _ => BodyElement::Raw(raw::RawElement::parse(node, opts)?),
        };
        Ok(res)
    }
}
