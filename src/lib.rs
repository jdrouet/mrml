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
    element.set_context(util::Context::default());
    Ok(element.render()?)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn clean_str(input: String) -> String {
        input
            .replace("class=\"\"", "")
            .replace("style=\"\"", "")
            .replace("\n", "")
            .replace(" ", "")
    }

    pub fn compare_render(source: &str, expected: &str) {
        let result = to_html(source);
        assert_eq!(result.is_ok(), true);
        let result = clean_str(result.unwrap());
        let expected = clean_str(expected.into());
        assert_diff!(result.as_str(), expected.as_str(), "", 0);
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
