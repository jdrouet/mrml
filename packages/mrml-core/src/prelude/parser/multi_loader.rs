//! Module containing a loader that is composed of multiple loaders.

use super::loader::IncludeLoaderError;
use crate::prelude::parser::loader::IncludeLoader;

#[derive(Debug, Default)]
/// This struct is a
/// [`IncludeLoader`](crate::prelude::parser::loader::IncludeLoader) where
/// you can define a strategy of resolver depending on the path.
/// That way, you can have a resolver for paths starting with `https://` and
/// another resolver for local files where the paths start with `file://`.
/// If no provider match the path, a `NotFound` error will be returned.
///
/// # Example
/// ```rust
/// use std::sync::Arc;
/// use mrml::mj_include::body::MjIncludeBodyKind;
/// use mrml::prelude::parser::memory_loader::MemoryIncludeLoader;
/// use mrml::prelude::parser::multi_loader::MultiIncludeLoader;
/// use mrml::prelude::parser::noop_loader::NoopIncludeLoader;
/// use mrml::prelude::parser::ParserOptions;
///
/// let resolver = MultiIncludeLoader::default()
///     .with_starts_with("file://", Box::new(MemoryIncludeLoader::from(vec![("file://basic.mjml", "<mj-button>Hello</mj-button>")])))
///     .with_any(Box::new(NoopIncludeLoader));
/// let opts = ParserOptions {
///     include_loader: Box::new(resolver),
/// };
/// let json = r#"<mjml>
///   <mj-body>
///     <mj-include path="file://basic.mjml" />
///   </mj-body>
/// </mjml>"#;
/// match mrml::parse_with_options(json, Arc::new(opts)) {
///     Ok(_) => println!("Success!"),
///     Err(err) => eprintln!("Couldn't parse template: {err:?}"),
/// }
/// ```
pub struct MultiIncludeLoader(Vec<MultiIncludeLoaderItem>);

impl MultiIncludeLoader {
    fn with_item(
        mut self,
        filter: MultiIncludeLoaderFilter,
        loader: Box<dyn IncludeLoader + Sync + Send + 'static>,
    ) -> Self {
        self.0.push(MultiIncludeLoaderItem { filter, loader });
        self
    }

    pub fn with_any(self, loader: Box<dyn IncludeLoader + Sync + Send + 'static>) -> Self {
        self.with_item(MultiIncludeLoaderFilter::Any, loader)
    }

    pub fn with_starts_with<S: ToString>(
        self,
        starts_with: S,
        loader: Box<dyn IncludeLoader + Sync + Send + 'static>,
    ) -> Self {
        self.with_item(
            MultiIncludeLoaderFilter::StartsWith {
                value: starts_with.to_string(),
            },
            loader,
        )
    }

    fn add_item(
        &mut self,
        filter: MultiIncludeLoaderFilter,
        loader: Box<dyn IncludeLoader + Sync + Send + 'static>,
    ) {
        self.0.push(MultiIncludeLoaderItem { filter, loader });
    }

    pub fn add_any(&mut self, loader: Box<dyn IncludeLoader + Sync + Send + 'static>) {
        self.add_item(MultiIncludeLoaderFilter::Any, loader);
    }

    pub fn add_starts_with<S: ToString>(
        &mut self,
        starts_with: S,
        loader: Box<dyn IncludeLoader + Sync + Send + 'static>,
    ) {
        self.add_item(
            MultiIncludeLoaderFilter::StartsWith {
                value: starts_with.to_string(),
            },
            loader,
        );
    }
}

#[derive(Debug)]
enum MultiIncludeLoaderFilter {
    StartsWith { value: String },
    Any,
}

impl MultiIncludeLoaderFilter {
    pub fn matches(&self, path: &str) -> bool {
        match self {
            Self::Any => true,
            Self::StartsWith { value } => path.starts_with(value),
        }
    }
}

#[derive(Debug)]
struct MultiIncludeLoaderItem {
    pub filter: MultiIncludeLoaderFilter,
    pub loader: Box<dyn IncludeLoader + Sync + Send + 'static>,
}

#[cfg_attr(feature = "async", async_trait::async_trait(?Send))]
impl IncludeLoader for MultiIncludeLoader {
    fn resolve(&self, path: &str) -> Result<String, IncludeLoaderError> {
        self.0
            .iter()
            .find(|item| item.filter.matches(path))
            .ok_or_else(|| {
                IncludeLoaderError::not_found(path)
                    .with_message("unable to find a compatible resolver")
            })
            .and_then(|item| item.loader.resolve(path))
    }

    #[cfg(feature = "async")]
    async fn async_resolve(&self, path: &str) -> Result<String, IncludeLoaderError> {
        let item = self
            .0
            .iter()
            .find(|item| item.filter.matches(path))
            .ok_or_else(|| {
                IncludeLoaderError::not_found(path)
                    .with_message("unable to find a compatible resolver")
            })?;
        item.loader.async_resolve(path).await
    }
}

#[cfg(test)]
mod tests {
    use std::io::ErrorKind;

    #[test]
    fn should_resolve() {
        use crate::prelude::parser::loader::IncludeLoader;
        use crate::prelude::parser::memory_loader::MemoryIncludeLoader;
        use crate::prelude::parser::multi_loader::MultiIncludeLoader;
        use crate::prelude::parser::noop_loader::NoopIncludeLoader;

        let resolver = MultiIncludeLoader::default()
            .with_starts_with(
                "file://",
                Box::new(MemoryIncludeLoader::from(vec![(
                    "file://basic.mjml",
                    "<mj-button>Hello</mj-button>",
                )])),
            )
            .with_any(Box::<NoopIncludeLoader>::default());

        assert_eq!(
            resolver.resolve("file://basic.mjml").unwrap(),
            "<mj-button>Hello</mj-button>"
        );

        let err = resolver.resolve("file://not-found.mjml").unwrap_err();
        assert_eq!(err.reason, ErrorKind::NotFound);
        // assert_eq!(err.message.unwrap(), "unable to find compatible resolver");

        let err = resolver.resolve("noop://not-found.mjml").unwrap_err();
        assert_eq!(err.reason, ErrorKind::NotFound);
        assert!(err.message.is_none());
    }

    #[test]
    fn should_not_find_resolver() {
        use crate::prelude::parser::loader::IncludeLoader;
        use crate::prelude::parser::multi_loader::MultiIncludeLoader;

        let resolver = MultiIncludeLoader::default();

        let err = resolver.resolve("file://not-found.mjml").unwrap_err();
        assert_eq!(err.reason, ErrorKind::NotFound);
        assert_eq!(err.message.unwrap(), "unable to find a compatible resolver");
    }

    #[test]
    fn should_build_resolvers() {
        use crate::prelude::parser::multi_loader::MultiIncludeLoader;
        use crate::prelude::parser::noop_loader::NoopIncludeLoader;

        let mut resolver = MultiIncludeLoader::default();
        resolver.add_starts_with("foo", Box::<NoopIncludeLoader>::default());
        resolver.add_any(Box::<NoopIncludeLoader>::default());
        assert_eq!(resolver.0.len(), 2);

        assert_eq!(format!("{resolver:?}"), "MultiIncludeLoader([MultiIncludeLoaderItem { filter: StartsWith { value: \"foo\" }, loader: NoopIncludeLoader }, MultiIncludeLoaderItem { filter: Any, loader: NoopIncludeLoader }])");
    }
}
