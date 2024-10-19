#![cfg(feature = "parse")]

#[test]
fn should_preserve_comment() {
    let template = r#"<mjml>
    <mj-head>
    </mj-head>
    <mj-body>
    </mj-body> <!-- Here -->
</mjml>"#;

    let _root = mrml::parse(template).unwrap();
}
