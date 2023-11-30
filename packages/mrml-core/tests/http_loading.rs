#[cfg(any(
    feature = "http-loader-async-reqwest",
    feature = "http-loader-blocking-reqwest"
))]
const TEMPLATE_URL: &str = "https://gist.githubusercontent.com/jdrouet/b0ac80fa08a3e7262bd4c94fc8865a87/raw/ec8771f4804a6c38427ed2a9f5937e11ec2b8c27/hello-world.mjml";

#[cfg(feature = "http-loader-async-reqwest")]
#[tokio::test]
async fn async_loading_include() {
    use std::collections::HashSet;
    use std::sync::Arc;

    use mrml::prelude::parser::http_loader::{AsyncReqwestFetcher, HttpIncludeLoader};
    use mrml::prelude::parser::AsyncParserOptions;

    let template = format!("<mjml><mj-body><mj-include path={TEMPLATE_URL:?} /></mj-body></mjml>");
    let resolver = HttpIncludeLoader::<AsyncReqwestFetcher>::new_allow(HashSet::from([
        "https://gist.githubusercontent.com".to_string(),
    ]));
    let options = Arc::new(AsyncParserOptions {
        include_loader: Arc::new(resolver),
    });
    let _ = mrml::async_parse_with_options(template, options)
        .await
        .unwrap();
}

#[cfg(feature = "http-loader-blocking-reqwest")]
#[test]
fn sync_loading_include() {
    use std::collections::HashSet;
    use std::sync::Arc;

    use mrml::prelude::parser::http_loader::{BlockingReqwestFetcher, HttpIncludeLoader};
    use mrml::prelude::parser::ParserOptions;

    let template = format!("<mjml><mj-body><mj-include path={TEMPLATE_URL:?} /></mj-body></mjml>");
    let resolver = HttpIncludeLoader::<BlockingReqwestFetcher>::new_allow(HashSet::from([
        "https://gist.githubusercontent.com".to_string(),
    ]));
    let options = Arc::new(ParserOptions {
        include_loader: Box::new(resolver),
    });
    let _ = mrml::parse_with_options(template, options).unwrap();
}
