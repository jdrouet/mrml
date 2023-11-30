// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod parser;
mod render;

use std::rc::Rc;

use wasm_bindgen::prelude::*;

pub use crate::parser::*;
pub use crate::render::*;

#[inline]
fn to_html(
    input: &str,
    parser_options: &mrml::prelude::parser::ParserOptions,
    render_options: &mrml::prelude::render::Options,
) -> Result<String, ToHtmlError> {
    let element = mrml::parse_with_options(input, parser_options)?;
    let html = element.render(render_options)?;
    Ok(html)
}

#[inline]
async fn to_html_async(
    input: &str,
    parser_options: Rc<mrml::prelude::parser::AsyncParserOptions>,
    render_options: &mrml::prelude::render::Options,
) -> Result<String, ToHtmlError> {
    let element = mrml::async_parse_with_options(input, parser_options).await?;
    let html = element.render(render_options)?;
    Ok(html)
}

#[derive(Debug, Default)]
#[wasm_bindgen]
pub struct Engine {
    parser: Rc<mrml::prelude::parser::ParserOptions>,
    #[cfg(feature = "async")]
    async_parser: Rc<mrml::prelude::parser::AsyncParserOptions>,
    render: mrml::prelude::render::Options,
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Defines the parsing options.
    #[allow(clippy::arc_with_non_send_sync)]
    #[wasm_bindgen(js_name = "setParserOptions")]
    pub fn set_parser_options(&mut self, value: ParserOptions) {
        self.parser = Rc::new(value.into());
    }

    /// Defines the async parsing options.
    #[allow(clippy::arc_with_non_send_sync)]
    #[wasm_bindgen(js_name = "setAsyncParserOptions")]
    pub fn set_async_parser_options(&mut self, value: AsyncParserOptions) {
        self.async_parser = Rc::new(value.into());
    }

    /// Defines the rendering options.
    #[wasm_bindgen(js_name = "setRenderOptions")]
    pub fn set_render_options(&mut self, value: RenderOptions) {
        self.render = value.into();
    }

    /// Renders the mjml input into html.
    #[wasm_bindgen(js_name = "toHtml")]
    pub fn to_html(&self, input: &str) -> ToHtmlResult {
        match to_html(input, &self.parser, &self.render) {
            Ok(content) => ToHtmlResult::Success { content },
            Err(error) => ToHtmlResult::Error(error),
        }
    }

    /// Renders the mjml input into html.
    #[cfg(feature = "async")]
    #[wasm_bindgen(js_name = "toHtmlAsync")]
    pub async fn to_html_async(&self, input: &str) -> ToHtmlResult {
        match to_html_async(input, self.async_parser.clone(), &self.render).await {
            Ok(content) => ToHtmlResult::Success { content },
            Err(error) => ToHtmlResult::Error(error),
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, tsify::Tsify)]
#[serde(rename_all = "camelCase", tag = "origin")]
#[tsify(into_wasm_abi)]
pub enum ToHtmlError {
    Parser { message: String },
    Render { message: String },
}

impl From<mrml::prelude::parser::Error> for ToHtmlError {
    fn from(value: mrml::prelude::parser::Error) -> Self {
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

impl ToHtmlResult {
    pub fn into_success(self) -> String {
        match self {
            Self::Success { content } => content,
            Self::Error(inner) => panic!("unexpected error {:?}", inner),
        }
    }
}

impl From<ToHtmlResult> for JsValue {
    fn from(value: ToHtmlResult) -> Self {
        serde_wasm_bindgen::to_value(&value).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Engine, ToHtmlResult};

    #[test]
    fn it_should_render_template() {
        let template = "<mjml><mj-body><mj-text>Hello World</mj-text></mj-body></mjml>";
        let opts = Engine::new();
        let result = opts.to_html(template);
        assert!(matches!(result, ToHtmlResult::Success { .. }));
    }
}
