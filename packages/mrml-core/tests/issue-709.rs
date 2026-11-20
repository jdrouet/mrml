#![cfg(all(feature = "parse", feature = "render", feature = "print"))]

use mrml::prelude::print::Printable;

const TEMPLATE: &str = "<mjml><mj-body><mj-section><mj-column><mj-image src=\"https://example.com/a.png?x=1&amp;y=2\" alt='Cafe\u{301} \u{1f44d}\u{fe0f}\ttab\\back \"quoted\"' /></mj-column></mj-section></mj-body></mjml>";

#[test]
fn should_render_attribute_values_verbatim() {
    let root = mrml::parse(TEMPLATE).unwrap();
    let html = root
        .element
        .render(&mrml::prelude::render::RenderOptions::default())
        .unwrap();
    assert!(
        html.contains("alt=\"Cafe\u{301} \u{1f44d}\u{fe0f}\ttab\\back &quot;quoted&quot;\""),
        "{html}"
    );
    assert!(html.contains("src=\"https://example.com/a.png?x=1&amp;y=2\""));
}

#[test]
fn should_print_attribute_values_verbatim() {
    let root = mrml::parse(TEMPLATE).unwrap();
    let mjml = root.element.print_dense().unwrap();
    assert!(
        mjml.contains("alt=\"Cafe\u{301} \u{1f44d}\u{fe0f}\ttab\\back &quot;quoted&quot;\""),
        "{mjml}"
    );
}
