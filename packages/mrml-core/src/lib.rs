pub mod comment;
pub mod mj_accordion;
pub mod mj_accordion_element;
pub mod mj_accordion_text;
pub mod mj_accordion_title;
pub mod mj_attributes;
pub mod mj_attributes_all;
pub mod mj_attributes_class;
pub mod mj_attributes_element;
pub mod mj_body;
pub mod mj_breakpoint;
pub mod mj_button;
pub mod mj_carousel;
pub mod mj_carousel_image;
pub mod mj_column;
pub mod mj_divider;
pub mod mj_font;
pub mod mj_group;
pub mod mj_head;
pub mod mj_hero;
pub mod mj_image;
pub mod mj_navbar;
pub mod mj_navbar_link;
pub mod mj_preview;
pub mod mj_raw;
pub mod mj_section;
pub mod mj_social;
pub mod mj_social_element;
pub mod mj_spacer;
pub mod mj_style;
pub mod mj_text;
pub mod mj_title;
pub mod mj_wrapper;
pub mod mjml;
pub mod node;
pub mod prelude;
pub mod text;

mod helper;
mod macros;

#[cfg(feature = "parse")]
pub fn parse<T: AsRef<str>>(input: T) -> Result<mjml::MJML, prelude::parse::Error> {
    mjml::MJML::parse(input)
}
