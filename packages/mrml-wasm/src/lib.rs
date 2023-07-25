// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use mrml::prelude::render::Options;
use wasm_bindgen::prelude::*;

#[derive(Debug, serde::Deserialize, serde::Serialize, tsify::Tsify)]
#[serde(rename_all = "camelCase", tag = "origin")]
#[tsify(into_wasm_abi)]
pub enum ToHtmlError {
    Parser { message: String },
    Render { message: String },
}

impl From<mrml::prelude::parse::Error> for ToHtmlError {
    fn from(value: mrml::prelude::parse::Error) -> Self {
        ToHtmlError::Parser {
            message: value.to_string(),
        }
    }
}

impl From<mrml::prelude::render::Error> for ToHtmlError {
    fn from(value: mrml::prelude::render::Error) -> Self {
        ToHtmlError::Render {
            message: value.to_string(),
        }
    }
}

#[derive(Debug, serde::Serialize, tsify::Tsify)]
#[serde(rename_all = "camelCase", tag = "type")]
#[tsify(into_wasm_abi)]
pub enum ToHtmlResult {
    Success { content: String },
    Error(ToHtmlError),
}

#[inline]
fn run_to_html(input: &str) -> Result<String, ToHtmlError> {
    let element = mrml::parse(input)?;
    let opts = Options::default();
    let html = element.render(&opts)?;
    Ok(html)
}

/// Parses the mjml template and renders it into html
#[wasm_bindgen]
pub fn to_html(input: &str) -> ToHtmlResult {
    match run_to_html(input) {
        Ok(content) => ToHtmlResult::Success { content },
        Err(error) => ToHtmlResult::Error(error),
    }
}

#[cfg(test)]
mod tests {
    use crate::{to_html, ToHtmlResult};

    #[test]
    fn it_should_render_template() {
        let template = "<mjml><mj-body><mj-text>Hello World</mj-text></mj-body></mjml>";
        let result = to_html(template);
        assert!(matches!(result, ToHtmlResult::Success { .. }));
    }
}
