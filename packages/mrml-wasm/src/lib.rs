#![allow(clippy::empty_docs)]

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
    render_options: &mrml::prelude::render::RenderOptions,
) -> Result<(String, Vec<Warning>), ToHtmlError> {
    let element = mrml::parse_with_options(input, parser_options)?;
    let html = element.element.render(render_options)?;
    Ok((html, Warning::from_vec(element.warnings)))
}

#[cfg(feature = "async")]
#[inline]
async fn to_html_async(
    input: &str,
    parser_options: std::sync::Arc<mrml::prelude::parser::AsyncParserOptions>,
    render_options: &mrml::prelude::render::RenderOptions,
) -> Result<(String, Vec<Warning>), ToHtmlError> {
    let element = mrml::async_parse_with_options(input, parser_options).await?;
    let html = element.element.render(render_options)?;
    Ok((html, Warning::from_vec(element.warnings)))
}

#[derive(Debug, Default)]
#[wasm_bindgen]
pub struct Engine {
    parser: Rc<mrml::prelude::parser::ParserOptions>,
    #[cfg(feature = "async")]
    async_parser: std::sync::Arc<mrml::prelude::parser::AsyncParserOptions>,
    render: mrml::prelude::render::RenderOptions,
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
    #[cfg(feature = "async")]
    #[allow(clippy::arc_with_non_send_sync)]
    #[wasm_bindgen(js_name = "setAsyncParserOptions")]
    pub fn set_async_parser_options(&mut self, value: AsyncParserOptions) {
        self.async_parser = std::sync::Arc::new(value.into());
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
            Ok((content, warnings)) => ToHtmlResult::Success { content, warnings },
            Err(error) => ToHtmlResult::Error(error),
        }
    }

    /// Renders the mjml input into html.
    #[cfg(feature = "async")]
    #[wasm_bindgen(js_name = "toHtmlAsync")]
    pub async fn to_html_async(&self, input: &str) -> ToHtmlResult {
        match to_html_async(input, self.async_parser.clone(), &self.render).await {
            Ok((content, warnings)) => ToHtmlResult::Success { content, warnings },
            Err(error) => ToHtmlResult::Error(error),
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, tsify::Tsify)]
#[serde(rename_all = "camelCase", tag = "origin")]
#[tsify(into_wasm_abi)]
pub enum ToHtmlError {
    Parser {
        message: String,
        details: ParserError,
    },
    Render {
        message: String,
    },
}

impl From<mrml::prelude::parser::Error> for ToHtmlError {
    fn from(value: mrml::prelude::parser::Error) -> Self {
        ToHtmlError::Parser {
            message: value.to_string(),
            details: value.into(),
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
    Success {
        content: String,
        warnings: Vec<Warning>,
    },
    Error(ToHtmlError),
}

impl ToHtmlResult {
    pub fn into_success(self) -> String {
        match self {
            Self::Success { content, .. } => content,
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
    use std::collections::HashMap;
    use std::iter::FromIterator;

    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{Engine, ToHtmlResult};

    #[wasm_bindgen_test]
    fn it_should_render() {
        let template = "<mjml><mj-body><mj-text>Hello World</mj-text></mj-body></mjml>";
        let opts = Engine::new();
        let result = opts.to_html(template);
        assert!(matches!(result, ToHtmlResult::Success { .. }));
    }

    #[wasm_bindgen_test]
    fn it_should_error() {
        let template = "<mjml><mj-body><mj-text>Hello World";
        let opts = Engine::new();
        let result = opts.to_html(template);
        assert!(matches!(result, ToHtmlResult::Error(_)));
    }

    #[wasm_bindgen_test]
    fn it_should_render_with_include() {
        let template = "<mjml><mj-body><mj-include path=\"/hello-world.mjml\" /></mj-body></mjml>";
        let mut opts = Engine::new();
        opts.set_parser_options(crate::ParserOptions {
            include_loader: crate::parser::IncludeLoaderOptions::Memory(
                crate::parser::MemoryIncludeLoaderOptions {
                    content: HashMap::from_iter([(
                        "/hello-world.mjml".to_string(),
                        "<mj-text>Hello World</mj-text>".to_string(),
                    )]),
                },
            ),
        });
        let result = opts.to_html(template);
        assert!(matches!(result, ToHtmlResult::Success { .. }));
    }
}

#[cfg(all(test, feature = "async"))]
mod async_tests {
    use std::collections::HashMap;
    use std::iter::FromIterator;

    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{Engine, ToHtmlResult};

    #[wasm_bindgen_test]
    async fn it_should_render() {
        let template = "<mjml><mj-body><mj-text>Hello World</mj-text></mj-body></mjml>";
        let opts = Engine::new();
        let result = opts.to_html_async(template).await;
        assert!(matches!(result, ToHtmlResult::Success { .. }));
    }

    #[wasm_bindgen_test]
    async fn it_should_error() {
        let template = "<mjml><mj-body><mj-text>Hello World";
        let opts = Engine::new();
        let result = opts.to_html_async(template).await;
        assert!(matches!(result, ToHtmlResult::Error(_)));
    }

    #[wasm_bindgen_test]
    async fn it_should_render_with_include() {
        let template = "<mjml><mj-body><mj-include path=\"/hello-world.mjml\" /></mj-body></mjml>";
        let mut opts = Engine::new();
        opts.set_async_parser_options(crate::AsyncParserOptions {
            include_loader: crate::parser::AsyncIncludeLoaderOptions::Memory(
                crate::parser::MemoryIncludeLoaderOptions {
                    content: HashMap::from_iter([(
                        "/hello-world.mjml".to_string(),
                        "<mj-text>Hello World</mj-text>".to_string(),
                    )]),
                },
            ),
        });
        let result = opts.to_html_async(template).await;
        assert!(matches!(result, ToHtmlResult::Success { .. }));
    }
}
