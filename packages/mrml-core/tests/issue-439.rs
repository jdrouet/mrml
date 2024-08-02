#![cfg(all(feature = "parse", feature = "render"))]

#[test]
fn should_preserve_whitespaces() {
    let template = r#"<mjml>
  <mj-body>
    <mj-text>
      <p>
        <em>foo</em> <strong>bar</strong>
      </p>
    </mj-text>
  </mj-body>
</mjml>"#;

    let root = mrml::parse(template).unwrap();
    let html = root
        .element
        .render(&mrml::prelude::render::RenderOptions::default())
        .unwrap();
    assert!(
        html.contains("<p><em>foo</em> <strong>bar</strong></p>"),
        "{}",
        html
    );
}
