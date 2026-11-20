#![cfg(all(feature = "parse", feature = "render"))]

fn render(template: &str) -> String {
    mrml::parse(template)
        .unwrap()
        .element
        .render(&mrml::prelude::render::RenderOptions::default())
        .unwrap()
}

#[test]
fn should_read_br_end_tag_as_br_in_mj_text() {
    let html = render(
        "<mjml><mj-body><mj-section><mj-column><mj-text><p>Saludos,<br></br><strong>Equipo</strong></p></mj-text></mj-column></mj-section></mj-body></mjml>",
    );
    assert!(
        html.contains("<p>Saludos,<br /><br /><strong>Equipo</strong></p>"),
        "{html}"
    );
}

#[test]
fn should_drop_other_void_end_tags_in_mj_text() {
    let html = render(
        "<mjml><mj-body><mj-section><mj-column><mj-text><p>a<hr></hr>b</p></mj-text></mj-column></mj-section></mj-body></mjml>",
    );
    assert!(html.contains("<p>a<hr />b</p>"), "{html}");
}

#[test]
fn should_read_br_end_tag_as_br_in_mj_raw() {
    let html = render("<mjml><mj-body><mj-raw><p>a<br></br>b</p></mj-raw></mj-body></mjml>");
    assert!(html.contains("<p>a<br /><br />b</p>"), "{html}");
}

#[cfg(feature = "async")]
#[tokio::test]
async fn should_read_br_end_tag_as_br_when_async() {
    let root = mrml::async_parse(
        "<mjml><mj-body><mj-section><mj-column><mj-text><p>a<br></br>b</p></mj-text></mj-column></mj-section></mj-body></mjml>",
    )
    .await
    .unwrap();
    let html = root
        .element
        .render(&mrml::prelude::render::RenderOptions::default())
        .unwrap();
    assert!(html.contains("<p>a<br /><br />b</p>"), "{html}");
}
