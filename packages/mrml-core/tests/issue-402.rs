#![cfg(feature = "parse")]

#[test]
fn should_parse_br_in_mj_raw() {
    let template = r#"<mjml>
    <mj-body>
        <mj-raw>something<br>else</mj-raw>
    </mj-body>
</mjml>"#;
    let _ = mrml::parse(template).unwrap();
}

#[test]
fn should_parse_br_in_mj_wrapper() {
    let template = r#"<mjml>
    <mj-body>
        <mj-wrapper>something<br>else</mj-wrapper>
    </mj-body>
</mjml>"#;
    let _ = mrml::parse(template).unwrap();
}
