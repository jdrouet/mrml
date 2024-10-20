#![cfg(feature = "parse")]

#[test]
fn should_allow_sendgrid_unsubscribe_tag() {
    let template = r#"<mjml>
<mj-body>
<mj-section>
<mj-column>
<mj-text>
<a href="<%asm_group_unsubscribe_raw_url%>">unsubscribe</a>
</mj-text>
</mj-column>
</mj-section>
</mj-body>
</mjml>"#;

    let _root = mrml::parse(template).unwrap();
}
