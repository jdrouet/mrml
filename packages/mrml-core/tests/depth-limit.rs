/// Regression tests for the two unbounded-recursion shapes found by the
/// security audit: deeply nested elements, and mutually-recursive
/// `mj-include` chains. Both used to abort the process with a stack
/// overflow instead of returning a parse error.
use mrml::prelude::parser::memory_loader::MemoryIncludeLoader;
use mrml::prelude::parser::{Error, ParserOptions};

const NESTED_DEPTH: usize = 20_000;

fn nested_divs(depth: usize) -> String {
    let mut template = String::from("<mjml><mj-body>");
    for _ in 0..depth {
        template.push_str("<div>");
    }
    for _ in 0..depth {
        template.push_str("</div>");
    }
    template.push_str("</mj-body></mjml>");
    template
}

fn assert_depth_limit_exceeded<T>(result: Result<T, Error>) {
    match result {
        Ok(_) => panic!("expected Err(DepthLimitExceeded), got Ok"),
        Err(err) => assert!(
            matches!(err, Error::DepthLimitExceeded { .. }),
            "expected DepthLimitExceeded, got a different error"
        ),
    }
}

#[test]
fn legitimate_nesting_still_parses() {
    // 60 nested elements is well within MAX_ELEMENT_DEPTH; this pins the
    // regression where a single shared depth counter rejected ordinary
    // WYSIWYG-editor output (~15 levels) long before any real stack risk.
    let template = nested_divs(60);
    mrml::parse(&template).expect("60 levels of nesting should parse");
}

#[test]
fn nested_elements_sync() {
    let template = nested_divs(NESTED_DEPTH);
    assert_depth_limit_exceeded(mrml::parse(&template));
}

#[tokio::test]
async fn nested_elements_async() {
    let template = nested_divs(NESTED_DEPTH);
    assert_depth_limit_exceeded(mrml::async_parse(&template).await);
}

fn cycle_opts() -> ParserOptions {
    ParserOptions {
        include_loader: Box::new(MemoryIncludeLoader::from(vec![
            (
                "a.mjml",
                r#"<mj-column><mj-include path="b.mjml" /></mj-column>"#,
            ),
            (
                "b.mjml",
                r#"<mj-column><mj-include path="a.mjml" /></mj-column>"#,
            ),
        ])),
    }
}

#[test]
fn mutual_include_cycle_sync() {
    let template = r#"<mjml><mj-body><mj-include path="a.mjml" /></mj-body></mjml>"#;
    assert_depth_limit_exceeded(mrml::parse_with_options(template, &cycle_opts()));
}

#[tokio::test]
async fn mutual_include_cycle_async() {
    use mrml::prelude::parser::AsyncParserOptions;

    let template = r#"<mjml><mj-body><mj-include path="a.mjml" /></mj-body></mjml>"#;
    let opts = std::sync::Arc::new(AsyncParserOptions {
        include_loader: Box::new(MemoryIncludeLoader::from(vec![
            (
                "a.mjml",
                r#"<mj-column><mj-include path="b.mjml" /></mj-column>"#,
            ),
            (
                "b.mjml",
                r#"<mj-column><mj-include path="a.mjml" /></mj-column>"#,
            ),
        ])),
    });
    assert_depth_limit_exceeded(mrml::async_parse_with_options(template, opts).await);
}
