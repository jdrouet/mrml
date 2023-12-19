#![cfg(feature = "parse")]

#[test]
fn should_parse_mjfont() {
    let template = r#"<mjml>
    <mj-head>
        <mj-font href="https://fonts.googleapis.com/css?family=Inter:normal&display=swap" name="Inter" />
        <mj-font href="https://fonts.googleapis.com/css?family=Inter:bold&display=swap" name="InterBold">
        </mj-font>
    </mj-head>
    <mj-body>
        <div>Hello World</div>
    </mj-body>
</mjml>"#;
    let _ = mrml::parse(template).unwrap();
}

#[test]
fn should_parse_mjimage() {
    let template = r#"<mjml>
    <mj-body>
        <div>Hello World</div>
        <mj-image src="/other.png" />
        <mj-image src="/wherever.png">  </mj-image>
        <mj-image src="/wherever.png">
        </mj-image>
    </mj-body>
</mjml>"#;
    let _ = mrml::parse(template).unwrap();
}
