#[cfg(test)]
#[macro_use(assert_diff)]
extern crate difference;

use roxmltree::Document;

mod error;
mod macros;
pub mod mjml;
pub mod util;

pub use error::Error;
use mjml::prelude::Component;

pub fn to_html(input: &str) -> Result<String, Error> {
    let doc = Document::parse(input)?;
    let root = doc.root_element();
    let mut element = mjml::Element::parse(root)?;
    element.set_context(util::Properties::new());
    Ok(element.render()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clean_str(input: String) -> String {
        input
            .replace("class=\"\"", "")
            .replace("style=\"\"", "")
            .replace("\n", "")
            .replace(" ", "")
    }

    fn compare_html(result: String, expected: String) {
        let result = clean_str(result);
        let expected = clean_str(expected);
        assert_diff!(result.as_str(), expected.as_str(), "", 0);
    }

    #[test]
    fn parse_render_mjml() {
        let source = include_str!("../test/mjml.mjml");
        let expected = include_str!("../test/mjml.html");
        let result = to_html(source);
        assert_eq!(result.is_ok(), true);
        compare_html(result.unwrap(), expected.into());
    }

    #[test]
    fn parse_render_mj_body() {
        let source = include_str!("../test/mj-body.mjml");
        let expected = include_str!("../test/mj-body.html");
        let result = to_html(source);
        assert_eq!(result.is_ok(), true);
        compare_html(result.unwrap(), expected.into());
    }

    #[test]
    fn parse_render_mj_body_with_background_color() {
        let source = include_str!("../test/mj-body-background-color.mjml");
        let expected = include_str!("../test/mj-body-background-color.html");
        let result = to_html(source);
        assert_eq!(result.is_ok(), true);
        compare_html(result.unwrap(), expected.into());
    }

    #[test]
    fn parse_render_mj_body_with_class() {
        let source = include_str!("../test/mj-body-class.mjml");
        let expected = include_str!("../test/mj-body-class.html");
        let result = to_html(source);
        assert_eq!(result.is_ok(), true);
        compare_html(result.unwrap(), expected.into());
    }

    #[test]
    fn parse_render_mj_body_with_width() {
        let source = include_str!("../test/mj-body-width.mjml");
        let expected = include_str!("../test/mj-body-width.html");
        let result = to_html(source);
        assert_eq!(result.is_ok(), true);
        compare_html(result.unwrap(), expected.into());
    }

    #[test]
    fn parse_render_mj_section_with_body_width() {
        let source = include_str!("../test/mj-section-body-width.mjml");
        let expected = include_str!("../test/mj-section-body-width.html");
        let result = to_html(source);
        assert_eq!(result.is_ok(), true);
        compare_html(result.unwrap(), expected.into());
    }

    #[test]
    fn parse_render_mj_section() {
        let source = include_str!("../test/mj-section.mjml");
        let expected = include_str!("../test/mj-section.html");
        let result = to_html(source);
        assert_eq!(result.is_ok(), true);
        compare_html(result.unwrap(), expected.into());
    }

    #[test]
    fn parse_render_mj_section_with_background_color() {
        let source = include_str!("../test/mj-section-background-color.mjml");
        let expected = include_str!("../test/mj-section-background-color.html");
        let result = to_html(source);
        assert_eq!(result.is_ok(), true);
        compare_html(result.unwrap(), expected.into());
    }

    #[test]
    fn parse_render_mj_section_with_background_url() {
        let source = include_str!("../test/mj-section-background-url.mjml");
        let expected = include_str!("../test/mj-section-background-url.html");
        let result = to_html(source);
        assert_eq!(result.is_ok(), true);
        compare_html(result.unwrap(), expected.into());
    }

    // #[test]
    // fn mjml_to_html_hello_world() {
    //     let source = include_str!("../test/hello-world.mjml");
    //     let expected = include_str!("../test/hello-world.html");
    //     let result = to_html(source);
    //     assert_eq!(result.is_ok(), true);
    //     assert_eq!(result.unwrap(), expected);
    // }
}
