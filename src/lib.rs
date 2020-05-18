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
use util::fonts::FontRegistry;
use util::Size;

#[derive(Clone, Debug)]
pub struct Options {
    pub breakpoint: Size,
    pub fonts: FontRegistry,
    pub keep_comments: bool,
}

impl Options {
    pub fn default() -> Self {
        Self {
            breakpoint: Size::Pixel(480.0),
            fonts: FontRegistry::default(),
            keep_comments: true,
        }
    }
}

pub fn to_html(input: &str, options: Options) -> Result<String, Error> {
    let doc = Document::parse(input)?;
    let root = doc.root_element();
    let mut element = mjml::Element::parse(root)?;
    element.set_context(util::Context::default(options));
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
            .replace("<![endif]--><!--[ifmso|IE]>", "")
    }

    pub fn compare_render(source: &str, expected: &str) {
        let result = to_html(source, Options::default()).unwrap();
        let result = clean_str(result);
        let expected = clean_str(expected.into());
        assert_diff!(result.as_str(), expected.as_str(), "", 0);
    }

    pub fn compare_render_with_options(source: &str, expected: &str, opts: Options) {
        let result = to_html(source, opts).unwrap();
        let result = clean_str(result);
        let expected = clean_str(expected.into());
        assert_diff!(result.as_str(), expected.as_str(), "", 0);
    }
}
