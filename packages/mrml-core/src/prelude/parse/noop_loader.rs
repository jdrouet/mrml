//! Module containing a loader that doesn't load any template.

use crate::prelude::parse::loader::IncludeLoader;

use super::loader::IncludeLoaderError;

#[derive(Debug, Default)]
/// This struct is a simple [`IncludeLoader`](crate::prelude::parse::loader::IncludeLoader) that doesn't
/// resolve any template. This is the default loader.
///
/// # Example
/// ```rust
/// use std::rc::Rc;
/// use mrml::mj_include::body::MjIncludeBodyKind;
/// use mrml::prelude::parse::noop_loader::NoopIncludeLoader;
/// use mrml::prelude::parse::ParserOptions;
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
/// match mrml::parse_with_options(json, Rc::new(opts)) {
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
