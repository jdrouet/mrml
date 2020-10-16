use wasm_bindgen::prelude::*;

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
