#![cfg(all(feature = "parse", feature = "render"))]

fn render(template: &str) -> String {
    mrml::parse(template)
        .unwrap()
        .element
        .render(&mrml::prelude::render::RenderOptions::default())
        .unwrap()
}

#[test]
fn should_render_unclosed_void_element_in_mj_raw() {
    let html = render(
        "<mjml>\n  <mj-body>\n    <mj-raw>\n      <img src=\"bananas\">\n    </mj-raw>\n  </mj-body>\n</mjml>",
    );
    assert!(html.contains("<img src=\"bananas\" />"), "{html}");
}

#[test]
fn should_render_unclosed_br_in_mj_raw() {
    let html = render(
        "<mjml><mj-body><mj-raw><address><strong>Foo</strong><br>bar<br>baz</address></mj-raw></mj-body></mjml>",
    );
    assert!(
        html.contains("<address><strong>Foo</strong><br />bar<br />baz</address>"),
        "{html}"
    );
}
