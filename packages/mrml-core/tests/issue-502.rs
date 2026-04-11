/// Regression test for https://github.com/jdrouet/mrml/issues/502
/// mj-include should handle included templates with multiple root elements.
#[test]
fn mj_include_multiple_root_elements() {
    let resolver = mrml::prelude::parser::memory_loader::MemoryIncludeLoader::from(vec![(
        "buttons.mjml",
        "<mj-button>Test 1</mj-button>\n<mj-button>Test 2</mj-button>",
    )]);
    let opts = mrml::prelude::parser::ParserOptions {
        include_loader: Box::new(resolver),
    };
    let template = r#"<mjml>
  <mj-body>
    <mj-section>
      <mj-column>
        <mj-include path="buttons.mjml" />
      </mj-column>
    </mj-section>
  </mj-body>
</mjml>"#;

    let doc = mrml::parse_with_options(template, &opts).expect("should parse successfully");
    let html = doc
        .element
        .render(&Default::default())
        .expect("should render successfully");
    assert!(html.contains("Test 1"));
    assert!(html.contains("Test 2"));
}
