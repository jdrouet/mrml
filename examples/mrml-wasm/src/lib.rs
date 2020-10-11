use mrml;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn to_html(input: &str) -> Option<String> {
    match mrml::to_html(input, mrml::Options::default()) {
        Ok(res) => Some(res),
        Err(err) => {
            console::log_1(&JsValue::from_str(format!("error: {:?}", err).as_str()));
            None
        },
    }
}
