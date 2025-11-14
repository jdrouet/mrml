#![cfg(feature = "parse")]

const TEMPLATE: &str = r#"<!-- <Mailer.EmailView.template_mjml> lib/mailer/templates/template.mjml.heex:1 (ev2) --><!-- @caller lib/mailer/templates/template.mjml.heex:1 (ev2) --><!-- <Mailer.EmailView.layout_mjml> lib/mailer/templates/layout.mjml.heex:1 (ev2) --><mjml data-phx-loc="1">
  <mj-head data-phx-loc="2">
    <mj-preview data-phx-loc="3">test<!-- @caller lib/mailer/templates/layout.mjml.heex:3 (ev2) --></mj-preview>
  </mj-head>
</mjml>"#;

#[test]
fn should_parse() {
    let _root = mrml::parse(TEMPLATE).unwrap();
}
