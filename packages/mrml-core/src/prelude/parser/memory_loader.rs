//! Module containing a loader where all the possible files are stored in
//! memory.

use std::collections::HashMap;
use std::iter::FromIterator;

use super::loader::IncludeLoaderError;
use crate::prelude::hash::Map;
#[cfg(feature = "async")]
use crate::prelude::parser::loader::AsyncIncludeLoader;
use crate::prelude::parser::loader::IncludeLoader;

#[derive(Debug, Default)]
/// This struct is a simple [`IncludeLoader`] where you can store in a map all
/// files you want to be able to use with [`mj-include`](crate::mj_include).
///
/// # Example
/// ```rust
/// use mrml::mj_include::body::MjIncludeBodyKind;
/// use mrml::prelude::parser::memory_loader::MemoryIncludeLoader;
/// use mrml::prelude::parser::ParserOptions;
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
/// match mrml::parse_with_options(json, &opts) {
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

impl From<HashMap<String, String>> for MemoryIncludeLoader {
    fn from(value: HashMap<String, String>) -> Self {
        MemoryIncludeLoader(Map::from_iter(value))
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

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncIncludeLoader for MemoryIncludeLoader {
    async fn async_resolve(&self, path: &str) -> Result<String, IncludeLoaderError> {
        self.0
            .get(path)
            .cloned()
            .ok_or_else(|| IncludeLoaderError::not_found(path))
    }
}
