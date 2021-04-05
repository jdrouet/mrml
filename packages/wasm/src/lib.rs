use mrml::prelude::render::Options as RenderOptions;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Default)]
pub struct Options {
    inner: RenderOptions,
}

#[wasm_bindgen]
impl Options {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    #[wasm_bindgen(getter = disableComments)]
    pub fn disable_comments(&self) -> bool {
        self.inner.disable_comments
    }

    #[wasm_bindgen(setter = disableComments)]
    pub fn set_disable_comments(&mut self, field: bool) {
        self.inner.disable_comments = field;
    }

    #[wasm_bindgen(getter = socialIconOrigin)]
    pub fn social_icon_origin(&self) -> Option<String> {
        self.inner.social_icon_origin.clone()
    }

    #[wasm_bindgen(setter = socialIconOrigin)]
    pub fn set_social_icon_origin(&mut self, field: &str) {
        self.inner.social_icon_origin = Some(field.to_string());
    }
}

impl Options {
    pub fn inner(&self) -> &RenderOptions {
        &self.inner
    }
}

#[wasm_bindgen(js_name = toHtml)]
pub fn to_html(input: &str) -> Result<String, JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    let opts = Options::default();
    to_html_with_options(input, &opts)
}

#[wasm_bindgen(js_name = toHtmlWithOptions)]
pub fn to_html_with_options(input: &str, opts: &Options) -> Result<String, JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    mrml::parse(input)
        .map_err(|err| err.to_string())
        .and_then(|node| node.render(opts.inner()).map_err(|err| err.to_string()))
        .map_err(|msg| JsValue::from(msg))
}
