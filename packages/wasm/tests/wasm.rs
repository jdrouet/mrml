#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn pass() {
    let result = mrml_wasm::to_html("<mjml></mjml>");
    assert!(result.is_ok());
}

#[wasm_bindgen_test]
fn raise() {
    let result = mrml_wasm::to_html("<mjml");
    assert!(result.is_err());
}
