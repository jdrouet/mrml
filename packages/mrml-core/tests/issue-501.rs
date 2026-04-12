/// Regression tests for https://github.com/jdrouet/mrml/issues/501
/// mj-include inside mj-attributes should resolve included attribute defaults.
use mrml::prelude::parser::memory_loader::MemoryIncludeLoader;
use mrml::prelude::parser::ParserOptions;
use mrml::prelude::render::RenderOptions;

fn opts(entries: Vec<(&str, &str)>) -> ParserOptions {
    ParserOptions {
        include_loader: Box::new(MemoryIncludeLoader::from(entries)),
    }
}

/// mj-include inside mj-attributes should apply element defaults from the
/// included file (the "inside attributes" scenario from the issue).
#[test]
fn include_inside_mj_attributes_sync() {
    let opts = opts(vec![(
        "test.mjml",
        r#"<mj-button background-color="red" />"#,
    )]);
    let template = r#"<mjml>
  <mj-head>
    <mj-attributes>
      <mj-include path="test.mjml" />
    </mj-attributes>
  </mj-head>
  <mj-body>
    <mj-section>
      <mj-column>
        <mj-button>Click</mj-button>
      </mj-column>
    </mj-section>
  </mj-body>
</mjml>"#;
    let doc = mrml::parse_with_options(template, &opts).expect("should parse");
    let html = doc
        .element
        .render(&RenderOptions::default())
        .expect("should render");
    // The button should have the red background from the included attributes
    assert!(
        html.contains("background:red"),
        "included mj-attributes element default should be applied: {html}"
    );
}

/// Same test but using the async parser.
#[tokio::test]
async fn include_inside_mj_attributes_async() {
    let resolver = MemoryIncludeLoader::from(vec![(
        "test.mjml",
        r#"<mj-button background-color="red" />"#,
    )]);
    let opts = std::sync::Arc::new(mrml::prelude::parser::AsyncParserOptions {
        include_loader: Box::new(resolver),
    });
    let template = r#"<mjml>
  <mj-head>
    <mj-attributes>
      <mj-include path="test.mjml" />
    </mj-attributes>
  </mj-head>
  <mj-body>
    <mj-section>
      <mj-column>
        <mj-button>Click</mj-button>
      </mj-column>
    </mj-section>
  </mj-body>
</mjml>"#;
    let doc = mrml::async_parse_with_options(template, opts)
        .await
        .expect("should parse");
    let html = doc
        .element
        .render(&RenderOptions::default())
        .expect("should render");
    assert!(
        html.contains("background:red"),
        "included mj-attributes element default should be applied: {html}"
    );
}

/// Verify the second scenario from the issue (include at mj-head level wrapping
/// mj-attributes) still works correctly.
#[test]
fn include_wrapping_mj_attributes_still_works() {
    let opts = opts(vec![(
        "test.mjml",
        r#"<mj-attributes><mj-button background-color="red" /></mj-attributes>"#,
    )]);
    let template = r#"<mjml>
  <mj-head>
    <mj-include path="test.mjml" />
  </mj-head>
  <mj-body>
    <mj-section>
      <mj-column>
        <mj-button>Click</mj-button>
      </mj-column>
    </mj-section>
  </mj-body>
</mjml>"#;
    let doc = mrml::parse_with_options(template, &opts).expect("should parse");
    let html = doc
        .element
        .render(&RenderOptions::default())
        .expect("should render");
    assert!(
        html.contains("background:red"),
        "mj-include wrapping mj-attributes should still work: {html}"
    );
}

/// Include with multiple attribute elements in the partial.
#[test]
fn include_multiple_attribute_elements() {
    let opts = opts(vec![(
        "test.mjml",
        r#"<mj-all font-size="20px" /><mj-button background-color="red" />"#,
    )]);
    let template = r#"<mjml>
  <mj-head>
    <mj-attributes>
      <mj-include path="test.mjml" />
    </mj-attributes>
  </mj-head>
  <mj-body>
    <mj-section>
      <mj-column>
        <mj-button>Click</mj-button>
      </mj-column>
    </mj-section>
  </mj-body>
</mjml>"#;
    let doc = mrml::parse_with_options(template, &opts).expect("should parse");
    let html = doc
        .element
        .render(&RenderOptions::default())
        .expect("should render");
    assert!(
        html.contains("background:red"),
        "button background should be applied: {html}"
    );
    assert!(
        html.contains("font-size:20px"),
        "mj-all font-size should be applied: {html}"
    );
}

/// open/close mj-include with empty body should NOT load from file.
#[test]
fn open_close_include_with_no_children_does_not_load() {
    // The file contains attributes, but they should be ignored because
    // the include tag is in open/close form (not self-closing).
    let opts = opts(vec![(
        "test.mjml",
        r#"<mj-button background-color="red" />"#,
    )]);
    let template = r#"<mjml>
  <mj-head>
    <mj-attributes>
      <mj-include path="test.mjml"></mj-include>
    </mj-attributes>
  </mj-head>
  <mj-body>
    <mj-section>
      <mj-column>
        <mj-button>Click</mj-button>
      </mj-column>
    </mj-section>
  </mj-body>
</mjml>"#;
    let doc = mrml::parse_with_options(template, &opts).expect("should parse");
    let html = doc
        .element
        .render(&RenderOptions::default())
        .expect("should render");
    // Default button color is #414141, NOT red, because the include had inline
    // (empty) content so the file was not loaded.
    assert!(
        !html.contains("background:red"),
        "open/close include with empty body should not load file: {html}"
    );
}
