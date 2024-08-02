#![cfg(all(feature = "parse", feature = "render", feature = "local-loader"))]

use std::assert;

const TEMPLATE_PATH: &str = "file:///mj-text-hello-world.mjml";

#[test]
fn loading_include() {
    use mrml::prelude::parser::local_loader::LocalIncludeLoader;
    use mrml::prelude::parser::ParserOptions;
    use mrml::prelude::render::RenderOptions;

    let template = format!("<mjml><mj-body><mj-include path={TEMPLATE_PATH:?} /></mj-body></mjml>");
    let resolver = LocalIncludeLoader::new(
        std::env::current_dir()
            .unwrap()
            .join("tests")
            .join("resources"),
    );
    let options = ParserOptions {
        include_loader: Box::new(resolver),
    };
    let parsed = mrml::parse_with_options(template, &options).unwrap();
    let output = parsed.element.render(&RenderOptions::default()).unwrap();

    assert!(output.contains("Hello World"));
}
