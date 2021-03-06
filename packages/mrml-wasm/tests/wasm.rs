#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn to_html_pass() {
    let result = mrml_wasm::to_html("<mjml></mjml>");
    assert!(result.is_ok());
}

#[wasm_bindgen_test]
fn to_html_raise() {
    let result = mrml_wasm::to_html("<mjml");
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn to_html_with_options_pass() {
    let mut opts = mrml_wasm::Options::new();
    opts.set_disable_comments(true);
    opts.set_social_icon_origin("https://whatever.com/");
    let result = mrml_wasm::to_html_with_options("<mjml></mjml>", &opts);
    assert!(result.is_ok());
}

#[wasm_bindgen_test]
fn to_html_with_options_raise() {
    let mut opts = mrml_wasm::Options::new();
    opts.set_disable_comments(true);
    opts.set_social_icon_origin("https://whatever.com/");
    let result = mrml_wasm::to_html_with_options("<mjml", &opts);
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn validate_raise() {
    assert!(mrml_wasm::validate("<mjml").is_err());
}

#[wasm_bindgen_test]
fn validate_pass() {
    assert!(mrml_wasm::validate("<mjml></mjml>").is_ok());
}

#[wasm_bindgen_test]
fn to_json() {
    assert_eq!(
        mrml_wasm::to_json(r#"<mjml></mjml>"#, false).unwrap(),
        r#"{"type":"mjml"}"#
    );
    assert_eq!(
        mrml_wasm::to_json(r#"<mjml></mjml>"#, true).unwrap(),
        "{\n  \"type\": \"mjml\"\n}"
    );
}

#[wasm_bindgen_test]
fn to_mjml() {
    assert_eq!(
        mrml_wasm::to_mjml(r#"{"type":"mjml"}"#, false).unwrap(),
        r#"<mjml></mjml>"#
    );
    assert_eq!(
        mrml_wasm::to_mjml(r#"{"type":"mjml"}"#, true).unwrap(),
        "<mjml>\n</mjml>\n"
    );
}
