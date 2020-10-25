use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Default)]
pub struct Options {
    inner: mrml::Options,
}

#[wasm_bindgen]
impl Options {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    #[wasm_bindgen(getter)]
    pub fn breakpoint(&self) -> String {
        self.inner.breakpoint.to_string()
    }

    #[wasm_bindgen(setter)]
    pub fn set_breakpoint(&mut self, field: &str) {
        if let Ok(value) = mrml::util::size::Size::from_str(field) {
            self.inner.breakpoint = value;
        }
    }

    #[wasm_bindgen(getter = keepComments)]
    pub fn keep_comments(&self) -> bool {
        self.inner.keep_comments
    }

    #[wasm_bindgen(setter = keepComments)]
    pub fn set_keep_comments(&mut self, field: bool) {
        self.inner.keep_comments = field;
    }

    #[wasm_bindgen(getter = socialIconOrigin)]
    pub fn social_icon_origin(&self) -> String {
        self.inner.social_icon_origin.clone()
    }

    #[wasm_bindgen(setter = socialIconOrigin)]
    pub fn set_social_icon_origin(&mut self, field: &str) {
        self.inner.social_icon_origin = String::from(field);
    }
}

impl Options {
    pub fn inner(&self) -> mrml::Options {
        self.inner.clone()
    }
}

#[wasm_bindgen(js_name = toHtml)]
pub fn to_html(input: &str) -> Result<String, JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    match mrml::to_html(input, mrml::Options::default()) {
        Ok(value) => Ok(value),
        Err(err) => {
            let message = format!("{:?}", err);
            Err(JsValue::from(message.as_str()))
        }
    }
}

#[wasm_bindgen(js_name = toHtmlWithOptions)]
pub fn to_html_with_options(input: &str, opts: &Options) -> Result<String, JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    match mrml::to_html(input, opts.inner()) {
        Ok(value) => Ok(value),
        Err(err) => {
            let message = format!("{:?}", err);
            Err(JsValue::from(message.as_str()))
        }
    }
}
