#![cfg(feature = "parse")]

#[test]
fn should_parse_and_ignore_root_comment() {
    let template = r#"<!-- ignore me --><mjml><mj-body></mj-body></mjml>"#;
    let _ = mrml::parse(template).unwrap();
}
