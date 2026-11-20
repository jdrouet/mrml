#![allow(clippy::empty_docs)]

mod parser;
mod render;

use std::rc::Rc;

use tsify::{Ts, Tsify};
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
    pub fn set_parser_options(&mut self, value: Ts<ParserOptions>) -> Result<(), JsError> {
        self.parser = Rc::new(value.to_rust()?.into());
        Ok(())
    }

    /// Defines the async parsing options.
    #[cfg(feature = "async")]
    #[allow(clippy::arc_with_non_send_sync)]
    #[wasm_bindgen(js_name = "setAsyncParserOptions")]
    pub fn set_async_parser_options(
        &mut self,
        value: Ts<AsyncParserOptions>,
    ) -> Result<(), JsError> {
        self.async_parser = std::sync::Arc::new(value.to_rust()?.into());
        Ok(())
    }

    /// Defines the rendering options.
    #[wasm_bindgen(js_name = "setRenderOptions")]
    pub fn set_render_options(&mut self, value: Ts<RenderOptions>) -> Result<(), JsError> {
        self.render = value.to_rust()?.into();
        Ok(())
    }

    /// Renders the mjml input into html.
    #[wasm_bindgen(js_name = "toHtml")]
    pub fn to_html(&self, input: &str) -> Result<Ts<ToHtmlResult>, JsError> {
        Ok(ToHtmlResult::from(to_html(input, &self.parser, &self.render)).into_ts()?)
    }

    /// Renders the mjml input into html.
    #[cfg(feature = "async")]
    #[wasm_bindgen(js_name = "toHtmlAsync")]
    pub async fn to_html_async(&self, input: &str) -> Result<Ts<ToHtmlResult>, JsError> {
        let result = to_html_async(input, self.async_parser.clone(), &self.render).await;
        Ok(ToHtmlResult::from(result).into_ts()?)
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, tsify::Tsify)]
#[serde(rename_all = "camelCase", tag = "origin")]
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

#[derive(Debug, serde::Deserialize, serde::Serialize, tsify::Tsify)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ToHtmlResult {
    Success {
        content: String,
        warnings: Vec<Warning>,
    },
    Error(ToHtmlError),
}

impl From<Result<(String, Vec<Warning>), ToHtmlError>> for ToHtmlResult {
    fn from(value: Result<(String, Vec<Warning>), ToHtmlError>) -> Self {
        match value {
            Ok((content, warnings)) => Self::Success { content, warnings },
            Err(error) => Self::Error(error),
        }
    }
}

impl ToHtmlResult {
    pub fn into_success(self) -> String {
        match self {
            Self::Success { content, .. } => content,
            Self::Error(inner) => panic!("unexpected error {inner:?}"),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]

    use std::collections::HashMap;
    use std::iter::FromIterator;

    use tsify::Tsify;
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{Engine, ToHtmlResult};

    #[wasm_bindgen_test]
    fn it_should_render() {
        let template = "<mjml><mj-body><mj-text>Hello World</mj-text></mj-body></mjml>";
        let opts = Engine::new();
        let result = opts.to_html(template).unwrap().to_rust().unwrap();
        assert!(matches!(result, ToHtmlResult::Success { .. }));
    }

    #[wasm_bindgen_test]
    fn it_should_error() {
        let template = "<mjml><mj-body><mj-text>Hello World";
        let opts = Engine::new();
        let result = opts.to_html(template).unwrap().to_rust().unwrap();
        assert!(matches!(result, ToHtmlResult::Error(_)));
    }

    #[wasm_bindgen_test]
    fn it_should_render_with_include() {
        let template = "<mjml><mj-body><mj-include path=\"/hello-world.mjml\" /></mj-body></mjml>";
        let mut opts = Engine::new();
        opts.set_parser_options(
            crate::ParserOptions {
                include_loader: crate::parser::IncludeLoaderOptions::Memory(
                    crate::parser::MemoryIncludeLoaderOptions {
                        content: HashMap::from_iter([(
                            "/hello-world.mjml".to_string(),
                            "<mj-text>Hello World</mj-text>".to_string(),
                        )]),
                    },
                ),
            }
            .into_ts()
            .unwrap(),
        )
        .unwrap();
        let result = opts.to_html(template).unwrap().to_rust().unwrap();
        assert!(matches!(result, ToHtmlResult::Success { .. }));
    }
}

#[cfg(all(test, feature = "async"))]
mod async_tests {
    #![allow(dead_code)]

    use std::collections::HashMap;
    use std::iter::FromIterator;

    use tsify::Tsify;
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{Engine, ToHtmlResult};

    #[wasm_bindgen_test]
    async fn it_should_render() {
        let template = "<mjml><mj-body><mj-text>Hello World</mj-text></mj-body></mjml>";
        let opts = Engine::new();
        let result = opts
            .to_html_async(template)
            .await
            .unwrap()
            .to_rust()
            .unwrap();
        assert!(matches!(result, ToHtmlResult::Success { .. }));
    }

    #[wasm_bindgen_test]
    async fn it_should_error() {
        let template = "<mjml><mj-body><mj-text>Hello World";
        let opts = Engine::new();
        let result = opts
            .to_html_async(template)
            .await
            .unwrap()
            .to_rust()
            .unwrap();
        assert!(matches!(result, ToHtmlResult::Error(_)));
    }

    #[wasm_bindgen_test]
    async fn it_should_render_with_include() {
        let template = "<mjml><mj-body><mj-include path=\"/hello-world.mjml\" /></mj-body></mjml>";
        let mut opts = Engine::new();
        opts.set_async_parser_options(
            crate::AsyncParserOptions {
                include_loader: crate::parser::AsyncIncludeLoaderOptions::Memory(
                    crate::parser::MemoryIncludeLoaderOptions {
                        content: HashMap::from_iter([(
                            "/hello-world.mjml".to_string(),
                            "<mj-text>Hello World</mj-text>".to_string(),
                        )]),
                    },
                ),
            }
            .into_ts()
            .unwrap(),
        )
        .unwrap();
        let result = opts
            .to_html_async(template)
            .await
            .unwrap()
            .to_rust()
            .unwrap();
        assert!(matches!(result, ToHtmlResult::Success { .. }));
    }
}
