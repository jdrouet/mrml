//! Module containing a loader where all the possible files are stored in memory.

use super::loader::IncludeLoaderError;
use crate::prelude::hash::Map;
use crate::prelude::parse::loader::IncludeLoader;

#[derive(Debug, Default)]
/// This struct is a simple [`IncludeLoader`](crate::prelude::parse::loader::IncludeLoader) where
/// you can store in a map all files you want to be able to use with [`mj-include`](crate::mj_include).
///
/// # Example
/// ```rust
/// use std::rc::Rc;
/// use mrml::mj_include::body::MjIncludeBodyKind;
/// use mrml::prelude::parse::memory_loader::MemoryIncludeLoader;
/// use mrml::prelude::parse::ParserOptions;
///
/// let resolver = MemoryIncludeLoader::from(vec![("basic.mjml", "<mj-button>Hello</mj-button>")]);
/// let opts = ParserOptions {
///     include_loader: Box::new(resolver),
/// };
/// let json = r#"<mjml>
///   <mj-body>
///     <mj-include path="basic.mjml" />
///   </mj-body>
/// </mjml>"#;
/// match mrml::parse_with_options(json, Rc::new(opts)) {
///     Ok(_) => println!("Success!"),
///     Err(err) => eprintln!("Couldn't parse template: {err:?}"),
/// }
/// ```
pub struct MemoryIncludeLoader(pub Map<String, String>);

impl<K: ToString, V: ToString> From<Vec<(K, V)>> for MemoryIncludeLoader {
    fn from(value: Vec<(K, V)>) -> Self {
        let res = value
            .into_iter()
            .fold(Map::default(), |mut res, (key, value)| {
                res.insert(key.to_string(), value.to_string());
                res
            });
        MemoryIncludeLoader::from(res)
    }
}

impl From<Map<String, String>> for MemoryIncludeLoader {
    fn from(value: Map<String, String>) -> Self {
        MemoryIncludeLoader(value)
    }
}

impl IncludeLoader for MemoryIncludeLoader {
    fn resolve(&self, path: &str) -> Result<String, IncludeLoaderError> {
        self.0
            .get(path)
            .cloned()
            .ok_or_else(|| IncludeLoaderError::not_found(path))
    }
}
