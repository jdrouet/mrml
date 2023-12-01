//! Module containing a loader that doesn't load any template.

#[cfg(feature = "async")]
use super::loader::AsyncIncludeLoader;
use super::loader::IncludeLoaderError;
use crate::prelude::parser::loader::IncludeLoader;

#[derive(Debug, Default)]
/// This struct is a simple
/// [`IncludeLoader`](crate::prelude::parser::loader::IncludeLoader) that
/// doesn't resolve any template. This is the default loader.
///
/// # Example
/// ```rust
/// use mrml::mj_include::body::MjIncludeBodyKind;
/// use mrml::prelude::parser::noop_loader::NoopIncludeLoader;
/// use mrml::prelude::parser::ParserOptions;
///
/// // This could be done using `ParserOptions::default()`.
/// let opts = ParserOptions {
///     include_loader: Box::new(NoopIncludeLoader::default()),
/// };
/// let json = r#"<mjml>
///   <mj-body>
///     <mj-include path="basic.mjml" />
///   </mj-body>
/// </mjml>"#;
/// match mrml::parse_with_options(json, &opts) {
///     Ok(_) => eprintln!("This should not happen!"),
///     Err(err) => println!("Couldn't parse template: {err:?}"),
/// }
/// ```
pub struct NoopIncludeLoader;

impl IncludeLoader for NoopIncludeLoader {
    fn resolve(&self, path: &str) -> Result<String, IncludeLoaderError> {
        Err(IncludeLoaderError::not_found(path))
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait(?Send)]
impl AsyncIncludeLoader for NoopIncludeLoader {
    async fn async_resolve(&self, path: &str) -> Result<String, IncludeLoaderError> {
        Err(IncludeLoaderError::not_found(path))
    }
}
