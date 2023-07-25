//! Test suite for nodejs.

#![cfg(target_arch = "wasm32")]

use mrml_wasm::ToHtmlError;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn is_should_render_template() {
    let template = "<mjml><mj-body><mj-text>Hello World</mj-text></mj-body></mjml>";
    let engine = mrml_wasm::Engine::new();
    let result = engine.to_html(template);
    assert!(matches!(result, mrml_wasm::ToHtmlResult::Success { .. }));
}

#[wasm_bindgen_test]
fn is_should_fail_when_render_template() {
    let template = "<mjml><mj-body><mj-text>Hello World</mj-";
    let engine = mrml_wasm::Engine::new();
    let result = engine.to_html(template);
    match result {
        mrml_wasm::ToHtmlResult::Error(inner) => match inner {
            ToHtmlError::Parser { message } => {
                assert_eq!(message, "unable to load included template")
            }
            other => panic!("unexpected error {:?}", other),
        },
        _ => panic!("shouldn't compile"),
    }
}
