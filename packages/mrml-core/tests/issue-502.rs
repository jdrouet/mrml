/// Regression tests for https://github.com/jdrouet/mrml/issues/502
/// mj-include should handle included templates with multiple root elements.
use mrml::prelude::parser::memory_loader::MemoryIncludeLoader;
use mrml::prelude::parser::ParserOptions;

fn opts(entries: Vec<(&str, &str)>) -> ParserOptions {
    ParserOptions {
        include_loader: Box::new(MemoryIncludeLoader::from(entries)),
    }
}

fn body_template(include: &str) -> String {
    format!(
        r#"<mjml>
  <mj-body>
    <mj-section>
      <mj-column>
        {include}
      </mj-column>
    </mj-section>
  </mj-body>
</mjml>"#
    )
}

// -- happy-path: multiple root elements (sync) --------------------------

#[test]
fn multiple_root_elements_sync() {
    let opts = opts(vec![(
        "buttons.mjml",
        "<mj-button>Test 1</mj-button>\n<mj-button>Test 2</mj-button>",
    )]);
    let template = body_template(r#"<mj-include path="buttons.mjml" />"#);
    let doc = mrml::parse_with_options(&template, &opts).expect("should parse");
    let html = doc
        .element
        .render(&Default::default())
        .expect("should render");
    assert!(html.contains("Test 1"));
    assert!(html.contains("Test 2"));
}

// -- happy-path: multiple root elements (async) -------------------------

#[tokio::test]
async fn multiple_root_elements_async() {
    let resolver = MemoryIncludeLoader::from(vec![(
        "buttons.mjml",
        "<mj-button>Test 1</mj-button>\n<mj-button>Test 2</mj-button>",
    )]);
    let opts = std::sync::Arc::new(mrml::prelude::parser::AsyncParserOptions {
        include_loader: Box::new(resolver),
    });
    let template = body_template(r#"<mj-include path="buttons.mjml" />"#);
    let doc = mrml::async_parse_with_options(&template, opts)
        .await
        .expect("should parse");
    let html = doc
        .element
        .render(&Default::default())
        .expect("should render");
    assert!(html.contains("Test 1"));
    assert!(html.contains("Test 2"));
}

// -- single root still works (no regression) ----------------------------

#[test]
fn single_root_element_still_works() {
    let opts = opts(vec![("button.mjml", "<mj-button>Only one</mj-button>")]);
    let template = body_template(r#"<mj-include path="button.mjml" />"#);
    let doc = mrml::parse_with_options(&template, &opts).expect("should parse");
    let html = doc
        .element
        .render(&Default::default())
        .expect("should render");
    assert!(html.contains("Only one"));
}

// -- multiple root elements of type="html" ------------------------------

#[test]
fn multiple_root_html_type() {
    let opts = opts(vec![("partial.html", "<h1>Hello</h1><p>World</p>")]);
    let template = body_template(r#"<mj-include path="partial.html" type="html" />"#);
    let doc = mrml::parse_with_options(&template, &opts).expect("should parse");
    let html = doc
        .element
        .render(&Default::default())
        .expect("should render");
    assert!(html.contains("Hello"));
    assert!(html.contains("World"));
}

// -- error positions are corrected (not shifted by the wrapper) ---------

#[test]
fn error_position_is_correct_for_invalid_include_child() {
    let opts = opts(vec![("bad.mjml", "<unknown-element />")]);
    let template = body_template(r#"<mj-include path="bad.mjml" />"#);
    match mrml::parse_with_options(&template, &opts) {
        Ok(_) => panic!("should have failed to parse"),
        Err(err) => {
            let msg = err.to_string();
            // The error should reference the include file.
            assert!(
                msg.contains("bad.mjml"),
                "error should reference the include path: {msg}"
            );
            // "unknown-element" starts at byte 1 in "<unknown-element />".
            // The position must NOT be shifted by the 15-byte wrapper.
            assert!(
                msg.contains("1:"),
                "error position start should not be shifted by the wrapper: {msg}"
            );
        }
    }
}

// -- warning positions inside wrapped include are correct ---------------

#[test]
fn warning_positions_corrected_in_wrapped_include() {
    // mj-raw with an unknown attribute "foo" triggers a warning.
    let opts = opts(vec![(
        "warn.html",
        r#"<mj-raw foo="bar"><p>Hi</p></mj-raw>"#,
    )]);
    let template = body_template(r#"<mj-include path="warn.html" type="html" />"#);
    let output = mrml::parse_with_options(&template, &opts).expect("should parse");
    assert_eq!(output.warnings.len(), 1);
    let w = &output.warnings[0];
    // "foo" attribute in the original content spans bytes 8..17.
    // The wrapper must NOT shift these positions.
    assert_eq!(
        w.to_string(),
        r#"unexpected attribute in template from "warn.html" at position 8:17"#,
    );
}
