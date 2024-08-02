pub mod body;
pub mod head;

pub const NAME: &str = "mj-include";

#[cfg(all(test, feature = "parse", feature = "render"))]
mod tests {
    use crate::mjml::Mjml;
    use crate::prelude::parser::memory_loader::MemoryIncludeLoader;
    use crate::prelude::parser::ParserOptions;
    use crate::prelude::render::RenderOptions;

    #[test]
    fn should_render_css_in_include() {
        let with_include = Mjml::parse_with_options(
            r#"<mjml>
<mj-head>
    <mj-include path="style.css" type="css" />
</mj-head>
<mj-body>
    <mj-text>Hello World!</mj-text>
</mj-body>
</mjml>"#,
            &ParserOptions {
                include_loader: Box::new(MemoryIncludeLoader::from(vec![(
                    "style.css",
                    ".container { background-color: #fffaee; padding: 48px 0px; }",
                )])),
            },
        )
        .unwrap();
        let basic = Mjml::parse(
            r#"<mjml>
<mj-head>
    <mj-style>.container { background-color: #fffaee; padding: 48px 0px; }</mj-style>
</mj-head>
<mj-body>
    <mj-text>Hello World!</mj-text>
</mj-body>
</mjml>"#,
        )
        .unwrap();

        let basic = basic.element.render(&RenderOptions::default()).unwrap();
        let with_include = with_include
            .element
            .render(&RenderOptions::default())
            .unwrap();
        similar_asserts::assert_eq!(basic, with_include);
    }

    #[test]
    fn should_render_mj_style_in_include() {
        let with_include = Mjml::parse_with_options(
            r#"<mjml>
<mj-head>
    <mj-include path="style.mjml" />
</mj-head>
<mj-body>
    <mj-text>Hello World!</mj-text>
</mj-body>
</mjml>"#,
            &ParserOptions {
                include_loader: Box::new(MemoryIncludeLoader::from(vec![(
                    "style.mjml",
                    r#"<mj-style>
.container { background-color: #fffaee; padding: 48px 0px; }
</mj-style>"#,
                )])),
            },
        )
        .unwrap();
        let basic = Mjml::parse(
            r#"<mjml>
<mj-head>
    <mj-style>.container { background-color: #fffaee; padding: 48px 0px; }</mj-style>
</mj-head>
<mj-body>
    <mj-text>Hello World!</mj-text>
</mj-body>
</mjml>"#,
        )
        .unwrap();

        let basic = basic.element.render(&RenderOptions::default()).unwrap();
        let with_include = with_include
            .element
            .render(&RenderOptions::default())
            .unwrap();
        similar_asserts::assert_eq!(basic, with_include);
    }
}
