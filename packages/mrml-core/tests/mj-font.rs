#![cfg(all(feature = "parse", feature = "render"))]

use mrml::prelude::render::RenderOptions;

#[test]
fn should_have_a_single_roboto_font_imported() {
    let parsed = mrml::parse(r#"<mjml>
  <mj-head>
    <mj-font name="Roboto" href="https://fonts.googleapis.com/css?family=Roboto:100,100i,300,300i,400,400i,500,500i,700,700i,900,900i" />
    </mj-head>
    <mj-body>
    <mj-section>
        <mj-column>
        <mj-text font-family="Roboto, Arial">
            Hello World!
        </mj-text>
        </mj-column>
    </mj-section>
    </mj-body>
</mjml>"#).unwrap();
    let rendered = parsed.element.render(&RenderOptions::default()).unwrap();
    assert!(!rendered.contains("Roboto:300,400,500,700"));
}

#[test]
fn quoted_font_family_value_still_imports_matching_font() {
    let parsed = mrml::parse(
        r#"<mjml>
  <mj-head>
    <mj-font name="Source Sans 3" href="https://fonts.googleapis.com/css?family=Source+Sans+3" />
    <mj-attributes>
      <mj-all font-family="'Source Sans 3', Helvetica, Arial, sans-serif" />
    </mj-attributes>
  </mj-head>
  <mj-body>
    <mj-section>
      <mj-column>
        <mj-text>Hello</mj-text>
      </mj-column>
    </mj-section>
  </mj-body>
</mjml>"#,
    )
    .unwrap();
    let rendered = parsed.element.render(&RenderOptions::default()).unwrap();
    assert!(rendered.contains("fonts.googleapis.com/css?family=Source+Sans+3"));
}
