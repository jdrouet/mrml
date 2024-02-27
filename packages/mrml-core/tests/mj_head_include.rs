#![cfg(feature = "parse")]

#[test]
fn should_apply_head_includes() {
    use mrml::{
        parse_with_options,
        prelude::{
            parser::{memory_loader::MemoryIncludeLoader, ParserOptions},
            render::RenderOptions,
        },
    };

    let include = r#"<mj-attributes>
        <mj-all font-family="serif" />
        <mj-class name="heading" color="red" />
    </mj-attributes>"#;
    let loader = MemoryIncludeLoader::from(vec![("mj-head-include-attributes.mjml", include)]);
    let parser_opts = ParserOptions {
        include_loader: Box::new(loader),
    };

    let render_opts = RenderOptions::default();
    let template = include_str!("resources/mj-head-include.mjml");
    let expected = include_str!("resources/mj-head-include.html");
    let root = parse_with_options(template, &parser_opts).unwrap();
    html_compare::assert_similar(expected, root.render(&render_opts).unwrap().as_str());
}
