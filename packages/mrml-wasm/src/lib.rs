// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use std::{borrow::Cow, collections::HashMap};

use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, tsify::Tsify)]
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
    render_options: &mrml::prelude::render::Options,
) -> Result<String, ToHtmlError> {
    let element = mrml::mjml::Mjml::parse(input)?;
    let html = element.render(render_options)?;
    Ok(html)
}

#[derive(Debug, Default)]
#[wasm_bindgen]
pub struct Engine {
    render: mrml::prelude::render::Options,
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Defines the rendering options.
    #[wasm_bindgen(js_name = "setRenderOptions")]
    pub fn set_render_options(&mut self, value: RenderOptions) {
        self.render = value.into();
    }

    /// Renders the mjml input into html.
    #[wasm_bindgen(js_name = "toHtml")]
    pub fn to_html(&self, input: &str) -> ToHtmlResult {
        match to_html(input, &self.render) {
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
