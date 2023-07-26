// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use std::{borrow::Cow, collections::HashMap, rc::Rc};

use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, tsify::Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct MemoryIncludeLoaderOptions {
    pub content: HashMap<String, String>,
}

impl MemoryIncludeLoaderOptions {
    pub fn build(self) -> Box<dyn mrml::prelude::parse::loader::IncludeLoader> {
        Box::new(mrml::prelude::parse::memory_loader::MemoryIncludeLoader::from(self.content))
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, tsify::Tsify)]
#[serde(tag = "type", rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum IncludeLoaderOptions {
    Noop,
    Memory(MemoryIncludeLoaderOptions),
}

impl Default for IncludeLoaderOptions {
    fn default() -> Self {
        Self::Noop
    }
}

impl IncludeLoaderOptions {
    pub fn build(self) -> Box<dyn mrml::prelude::parse::loader::IncludeLoader> {
        match self {
            Self::Noop => Box::new(mrml::prelude::parse::noop_loader::NoopIncludeLoader),
            Self::Memory(inner) => inner.build(),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, tsify::Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ParserOptions {
    pub include_loader: IncludeLoaderOptions,
}

impl From<ParserOptions> for mrml::prelude::parse::ParserOptions {
    fn from(value: ParserOptions) -> Self {
        mrml::prelude::parse::ParserOptions {
            include_loader: value.include_loader.build(),
        }
    }
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize, tsify::Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
/// Rendering options
pub struct RenderOptions {
    /// If disabled, the comments won't be kept in the result. Disabled by default.
    pub disable_comments: bool,
    /// Base url of the server to fetch the social icons.
    pub social_icon_origin: Option<String>,
    /// Map of fonts that can be used.
    pub fonts: HashMap<String, String>,
}

impl From<RenderOptions> for mrml::prelude::render::Options {
    fn from(value: RenderOptions) -> Self {
        Self {
            disable_comments: value.disable_comments,
            social_icon_origin: value.social_icon_origin.map(Cow::Owned),
            fonts: value
                .fonts
                .into_iter()
                .map(|(key, value)| (key, Cow::Owned(value)))
                .collect(),
        }
    }
}

#[inline]
fn to_html(
    input: &str,
    parser_options: Rc<mrml::prelude::parse::ParserOptions>,
    render_options: &mrml::prelude::render::Options,
) -> Result<String, ToHtmlError> {
    let element = mrml::parse_with_options(input, parser_options)?;
    let html = element.render(render_options)?;
    Ok(html)
}

#[derive(Debug, Default)]
#[wasm_bindgen]
pub struct Engine {
    parser: Rc<mrml::prelude::parse::ParserOptions>,
    render: mrml::prelude::render::Options,
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Defines the parsing options.
    #[wasm_bindgen(js_name = "setParserOptions")]
    pub fn set_parser_options(&mut self, value: ParserOptions) {
        self.parser = Rc::new(value.into());
    }

    /// Defines the rendering options.
    #[wasm_bindgen(js_name = "setRenderOptions")]
    pub fn set_render_options(&mut self, value: RenderOptions) {
        self.render = value.into();
    }

    /// Renders the mjml input into html.
    #[wasm_bindgen(js_name = "toHtml")]
    pub fn to_html(&self, input: &str) -> ToHtmlResult {
        match to_html(input, self.parser.clone(), &self.render) {
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

impl ToHtmlResult {
    pub fn into_success(self) -> String {
        match self {
            Self::Success { content } => content,
            Self::Error(inner) => panic!("unexpected error {:?}", inner),
        }
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
