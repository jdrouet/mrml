/// Regression tests for https://github.com/jdrouet/mrml/issues/503
/// mj-include should be usable directly inside <mjml>.
use mrml::prelude::parser::memory_loader::MemoryIncludeLoader;
use mrml::prelude::parser::ParserOptions;

fn opts(entries: Vec<(&str, &str)>) -> ParserOptions {
    ParserOptions {
        include_loader: Box::new(MemoryIncludeLoader::from(entries)),
    }
}

#[test]
fn include_head_inside_mjml_sync() {
    let opts = opts(vec![(
        "head.mjml",
        "<mj-head><mj-preview>Hello</mj-preview></mj-head>",
    )]);
    let template = r#"<mjml>
  <mj-include path="head.mjml" />
  <mj-body>
    <mj-section>
      <mj-column>
        <mj-text>World</mj-text>
      </mj-column>
    </mj-section>
  </mj-body>
</mjml>"#;
    let doc = mrml::parse_with_options(template, &opts).expect("should parse");
    assert!(doc.element.head().is_some());
    assert!(doc.element.body().is_some());
    let html = doc
        .element
        .render(&Default::default())
        .expect("should render");
    assert!(html.contains("Hello"));
    assert!(html.contains("World"));
}

#[tokio::test]
async fn include_head_inside_mjml_async() {
    let resolver = MemoryIncludeLoader::from(vec![(
        "head.mjml",
        "<mj-head><mj-preview>Hello</mj-preview></mj-head>",
    )]);
    let opts = std::sync::Arc::new(mrml::prelude::parser::AsyncParserOptions {
        include_loader: Box::new(resolver),
    });
    let template = r#"<mjml>
  <mj-include path="head.mjml" />
  <mj-body>
    <mj-section>
      <mj-column>
        <mj-text>World</mj-text>
      </mj-column>
    </mj-section>
  </mj-body>
</mjml>"#;
    let doc = mrml::async_parse_with_options(template, opts)
        .await
        .expect("should parse");
    assert!(doc.element.head().is_some());
    assert!(doc.element.body().is_some());
    let html = doc
        .element
        .render(&Default::default())
        .expect("should render");
    assert!(html.contains("Hello"));
    assert!(html.contains("World"));
}

#[test]
fn include_body_inside_mjml_sync() {
    let opts = opts(vec![(
        "body.mjml",
        "<mj-body><mj-section><mj-column><mj-text>Included</mj-text></mj-column></mj-section></mj-body>",
    )]);
    let template = r#"<mjml>
  <mj-head><mj-preview>Preview</mj-preview></mj-head>
  <mj-include path="body.mjml" />
</mjml>"#;
    let doc = mrml::parse_with_options(template, &opts).expect("should parse");
    assert!(doc.element.head().is_some());
    assert!(doc.element.body().is_some());
    let html = doc
        .element
        .render(&Default::default())
        .expect("should render");
    assert!(html.contains("Included"));
}

#[test]
fn include_head_and_body_inside_mjml_sync() {
    let opts = opts(vec![(
        "full.mjml",
        "<mj-head><mj-preview>Preview</mj-preview></mj-head><mj-body><mj-section><mj-column><mj-text>Body</mj-text></mj-column></mj-section></mj-body>",
    )]);
    let template = r#"<mjml>
  <mj-include path="full.mjml" />
</mjml>"#;
    let doc = mrml::parse_with_options(template, &opts).expect("should parse");
    assert!(doc.element.head().is_some());
    assert!(doc.element.body().is_some());
    let html = doc
        .element
        .render(&Default::default())
        .expect("should render");
    assert!(html.contains("Body"));
}
