//! Test suite for nodejs.

#![cfg(target_arch = "wasm32")]

use mrml_wasm::ToHtmlError;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn it_should_render_template() {
    let template = "<mjml><mj-body><mj-text>Hello World</mj-text></mj-body></mjml>";
    let engine = mrml_wasm::Engine::new();
    let result = engine.to_html(template);
    assert!(matches!(result, mrml_wasm::ToHtmlResult::Success { .. }));
}

#[cfg(feature = "async")]
#[wasm_bindgen_test]
async fn it_should_render_template_async() {
    let template = "<mjml><mj-body><mj-text>Hello World</mj-text></mj-body></mjml>";
    let engine = mrml_wasm::Engine::new();
    let result = engine.to_html_async(template).await;
    assert!(matches!(result, mrml_wasm::ToHtmlResult::Success { .. }));
}

#[wasm_bindgen_test]
fn it_should_fail_when_render_template() {
    let template = "<mjml><mj-body><mj-text>Hello World</mj-";
    let engine = mrml_wasm::Engine::new();
    let result = engine.to_html(template);
    match result {
        mrml_wasm::ToHtmlResult::Error(err) => match err {
            ToHtmlError::Parser {
                message,
                details: _,
            } => {
                assert_eq!(message, "unable to parse next template in root template")
            }
            other => panic!("unexpected error {:?}", other),
        },
        _ => panic!("shouldn't compile"),
    }
}

#[cfg(feature = "async")]
#[wasm_bindgen_test]
async fn it_should_fail_when_render_template_async() {
    let template = "<mjml><mj-body><mj-text>Hello World</mj-";
    let engine = mrml_wasm::Engine::new();
    let result = engine.to_html_async(template).await;
    match result {
        mrml_wasm::ToHtmlResult::Error(err) => match err {
            ToHtmlError::Parser {
                message,
                details: _,
            } => {
                assert_eq!(message, "unable to parse next template in root template")
            }
            other => panic!("unexpected error {:?}", other),
        },
        _ => panic!("shouldn't compile"),
    }
}

#[wasm_bindgen_test]
fn it_should_disable_comments() {
    let template = "<mjml><mj-body><mj-text>Hello World</mj-text><!-- Goodbye --></mj-body></mjml>";
    let mut engine = mrml_wasm::Engine::new();
    engine.set_render_options(mrml_wasm::RenderOptions {
        disable_comments: true,
        ..Default::default()
    });
    let result = engine.to_html(template);
    match result {
        mrml_wasm::ToHtmlResult::Success { content, warnings } => {
            assert_eq!(content.matches("Goodbye").count(), 0);
            assert!(warnings.is_empty())
        }
        err => panic!("shouldn't fail {:?}", err),
    }
}

#[cfg(feature = "async")]
#[wasm_bindgen_test]
async fn it_should_disable_comments_async() {
    let template = "<mjml><mj-body><mj-text>Hello World</mj-text><!-- Goodbye --></mj-body></mjml>";
    let mut engine = mrml_wasm::Engine::new();
    engine.set_render_options(mrml_wasm::RenderOptions {
        disable_comments: true,
        ..Default::default()
    });
    let result = engine.to_html_async(template).await;
    match result {
        mrml_wasm::ToHtmlResult::Success { content, warnings } => {
            assert_eq!(content.matches("Goodbye").count(), 0);
            assert!(warnings.is_empty());
        }
        err => panic!("shouldn't fail {:?}", err),
    }
}

#[wasm_bindgen_test]
fn it_should_use_noop_include_loader_by_default() {
    let template = "<mjml><mj-body><mj-text>Hello World</mj-text><mj-include path=\"./header.mjml\" /></mj-body></mjml>";
    let mut engine = mrml_wasm::Engine::new();
    engine.set_render_options(mrml_wasm::RenderOptions {
        disable_comments: true,
        ..Default::default()
    });
    let result = engine.to_html(template);
    match result {
        mrml_wasm::ToHtmlResult::Error(err) => match err {
            ToHtmlError::Parser {
                message,
                details: _,
            } => {
                assert_eq!(
                    message,
                    "unable to load included template in root template at position 46:56"
                )
            }
            other => panic!("unexpected error {:?}", other),
        },
        _ => panic!("shouldn't compile"),
    }
}

#[cfg(feature = "async")]
#[wasm_bindgen_test]
async fn it_should_use_noop_include_loader_by_default_async() {
    let template = "<mjml><mj-body><mj-text>Hello World</mj-text><mj-include path=\"./header.mjml\" /></mj-body></mjml>";
    let mut engine = mrml_wasm::Engine::new();
    engine.set_render_options(mrml_wasm::RenderOptions {
        disable_comments: true,
        ..Default::default()
    });
    let result = engine.to_html_async(template).await;
    match result {
        mrml_wasm::ToHtmlResult::Error(err) => match err {
            ToHtmlError::Parser {
                message,
                details: _,
            } => {
                assert_eq!(
                    message,
                    "unable to load included template in root template at position 46:56"
                )
            }
            other => panic!("unexpected error {:?}", other),
        },
        _ => panic!("shouldn't compile"),
    }
}

#[wasm_bindgen_test]
fn it_should_use_memory_include_loader_by_default() {
    let template = "<mjml><mj-body><mj-include path=\"./header.mjml\" /></mj-body></mjml>";
    let mut engine = mrml_wasm::Engine::new();
    engine.set_parser_options(mrml_wasm::ParserOptions {
        include_loader: mrml_wasm::IncludeLoaderOptions::Memory(
            mrml_wasm::MemoryIncludeLoaderOptions {
                content: [(
                    "./header.mjml".to_string(),
                    "<mj-text>Hello World</mj-text>".to_string(),
                )]
                .into(),
            },
        ),
    });
    engine.set_render_options(mrml_wasm::RenderOptions {
        disable_comments: true,
        ..Default::default()
    });
    let result = engine.to_html(template);
    let content = result.into_success();
    assert_eq!(content.matches("Hello World").count(), 1);
}

#[cfg(feature = "async")]
#[wasm_bindgen_test]
async fn it_should_use_memory_include_loader_by_default_async() {
    let template = "<mjml><mj-body><mj-include path=\"./header.mjml\" /></mj-body></mjml>";
    let mut engine = mrml_wasm::Engine::new();
    engine.set_async_parser_options(mrml_wasm::AsyncParserOptions {
        include_loader: mrml_wasm::AsyncIncludeLoaderOptions::Memory(
            mrml_wasm::MemoryIncludeLoaderOptions {
                content: [(
                    "./header.mjml".to_string(),
                    "<mj-text>Hello World</mj-text>".to_string(),
                )]
                .into(),
            },
        ),
    });
    engine.set_render_options(mrml_wasm::RenderOptions {
        disable_comments: true,
        ..Default::default()
    });
    let result = engine.to_html_async(template).await;
    let content = result.into_success();
    assert_eq!(content.matches("Hello World").count(), 1);
}
