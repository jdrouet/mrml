#![cfg(all(feature = "parse", feature = "render"))]

fn render(padding: &str) -> String {
    let template = format!(
        r#"<mjml><mj-body width="600px"><mj-section><mj-column><mj-image src="https://example.com/a.png" padding="{padding}" /><mj-divider padding="{padding}" /></mj-column></mj-section></mj-body></mjml>"#
    );
    let root = mrml::parse(template).unwrap();
    root.element
        .render(&mrml::prelude::render::RenderOptions::default())
        .unwrap()
}

#[test]
fn unitless_zero_padding_should_reduce_width() {
    let html = render("0 40px");
    assert!(html.contains(r#"width="520""#), "{html}");
    assert!(html.contains("width:520px"), "{html}");
    assert_eq!(html.replace("0 40px", "0px 40px"), render("0px 40px"));
}

#[test]
fn unitless_zero_column_padding_should_reduce_child_width() {
    let template = |padding: &str| {
        format!(
            r#"<mjml><mj-body width="600px"><mj-section><mj-column padding="{padding}"><mj-image src="https://example.com/a.png" /></mj-column></mj-section></mj-body></mjml>"#
        )
    };
    let render = |padding: &str| {
        mrml::parse(template(padding))
            .unwrap()
            .element
            .render(&mrml::prelude::render::RenderOptions::default())
            .unwrap()
    };
    let html = render("0 40px");
    assert!(html.contains(r#"width="470""#), "{html}");
    assert_eq!(html.replace("0 40px", "0px 40px"), render("0px 40px"));
}
