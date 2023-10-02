//! Module containing a loader that is composed of multiple loaders.

use super::loader::IncludeLoaderError;
use crate::prelude::parser::loader::IncludeLoader;

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
#[derive(Debug)]
pub struct MultiIncludeLoader<T> {
    inner: Vec<T>,
}

impl<T> Default for MultiIncludeLoader<T> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl MultiIncludeLoader<MultiIncludeLoaderItem> {
    fn with_item(
        mut self,
        filter: MultiIncludeLoaderFilter,
        loader: Box<dyn IncludeLoader>,
    ) -> Self {
        self.inner.push(MultiIncludeLoaderItem { filter, loader });
        self
    }

    pub fn with_any(self, loader: Box<dyn IncludeLoader>) -> Self {
        self.with_item(MultiIncludeLoaderFilter::Any, loader)
    }

    pub fn with_starts_with<S: ToString>(
        self,
        starts_with: S,
        loader: Box<dyn IncludeLoader>,
    ) -> Self {
        self.with_item(
            MultiIncludeLoaderFilter::StartsWith {
                value: starts_with.to_string(),
            },
            loader,
        )
    }

    fn add_item(&mut self, filter: MultiIncludeLoaderFilter, loader: Box<dyn IncludeLoader>) {
        self.inner.push(MultiIncludeLoaderItem { filter, loader });
    }

    pub fn add_any(&mut self, loader: Box<dyn IncludeLoader>) {
        self.add_item(MultiIncludeLoaderFilter::Any, loader);
    }

    pub fn add_starts_with<S: ToString>(&mut self, starts_with: S, loader: Box<dyn IncludeLoader>) {
        self.add_item(
            MultiIncludeLoaderFilter::StartsWith {
                value: starts_with.to_string(),
            },
            loader,
        );
    }
}

#[cfg(feature = "async-loader")]
impl MultiIncludeLoader<AsyncMultiIncludeLoaderItem> {
    fn with_item(
        mut self,
        filter: MultiIncludeLoaderFilter,
        loader: std::sync::Arc<dyn super::loader::AsyncIncludeLoader>,
    ) -> Self {
        self.inner
            .push(AsyncMultiIncludeLoaderItem { filter, loader });
        self
    }

    pub fn with_any(self, loader: std::sync::Arc<dyn super::loader::AsyncIncludeLoader>) -> Self {
        self.with_item(MultiIncludeLoaderFilter::Any, loader)
    }

    pub fn with_starts_with<S: ToString>(
        self,
        starts_with: S,
        loader: std::sync::Arc<dyn super::loader::AsyncIncludeLoader>,
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
        loader: std::sync::Arc<dyn super::loader::AsyncIncludeLoader>,
    ) {
        self.inner
            .push(AsyncMultiIncludeLoaderItem { filter, loader });
    }

    pub fn add_any(&mut self, loader: std::sync::Arc<dyn super::loader::AsyncIncludeLoader>) {
        self.add_item(MultiIncludeLoaderFilter::Any, loader);
    }

    pub fn add_starts_with<S: ToString>(
        &mut self,
        starts_with: S,
        loader: std::sync::Arc<dyn super::loader::AsyncIncludeLoader>,
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
pub struct MultiIncludeLoaderItem {
    filter: MultiIncludeLoaderFilter,
    loader: Box<dyn IncludeLoader>,
}

#[cfg(feature = "async-loader")]
#[derive(Debug)]
pub struct AsyncMultiIncludeLoaderItem {
    filter: MultiIncludeLoaderFilter,
    loader: std::sync::Arc<dyn super::loader::AsyncIncludeLoader>,
}

impl IncludeLoader for MultiIncludeLoader<MultiIncludeLoaderItem> {
    fn resolve(&self, path: &str) -> Result<String, IncludeLoaderError> {
        self.inner
            .iter()
            .find(|item| item.filter.matches(path))
            .ok_or_else(|| {
                IncludeLoaderError::not_found(path)
                    .with_message("unable to find a compatible resolver")
            })
            .and_then(|item| item.loader.resolve(path))
    }
}

#[cfg(feature = "async-loader")]
#[async_trait::async_trait]
impl super::loader::AsyncIncludeLoader for MultiIncludeLoader<AsyncMultiIncludeLoaderItem> {
    async fn resolve(&self, path: &str) -> Result<String, IncludeLoaderError> {
        let item = self
            .inner
            .iter()
            .find(|item| item.filter.matches(path))
            .ok_or_else(|| {
                IncludeLoaderError::not_found(path)
                    .with_message("unable to find a compatible resolver")
            })?;
        item.loader.resolve(path).await
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

        let resolver = MultiIncludeLoader::<super::MultiIncludeLoaderItem>::default()
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

        let resolver: MultiIncludeLoader<super::MultiIncludeLoaderItem> =
            MultiIncludeLoader::default();

        let err = resolver.resolve("file://not-found.mjml").unwrap_err();
        assert_eq!(err.reason, ErrorKind::NotFound);
        assert_eq!(err.message.unwrap(), "unable to find a compatible resolver");
    }

    #[test]
    fn should_build_resolvers() {
        use crate::prelude::parser::multi_loader::MultiIncludeLoader;
        use crate::prelude::parser::noop_loader::NoopIncludeLoader;

        let mut resolver: MultiIncludeLoader<super::MultiIncludeLoaderItem> =
            MultiIncludeLoader::default();
        resolver.add_starts_with("foo", Box::<NoopIncludeLoader>::default());
        resolver.add_any(Box::<NoopIncludeLoader>::default());
        assert_eq!(resolver.inner.len(), 2);

        assert_eq!(format!("{resolver:?}"), "MultiIncludeLoader { inner: [MultiIncludeLoaderItem { filter: StartsWith { value: \"foo\" }, loader: NoopIncludeLoader }, MultiIncludeLoaderItem { filter: Any, loader: NoopIncludeLoader }] }");
    }
}

#[cfg(all(test, feature = "async-loader"))]
mod async_tests {
    use std::io::ErrorKind;

    #[tokio::test]
    async fn should_resolve() {
        use std::sync::Arc;

        use crate::prelude::parser::loader::AsyncIncludeLoader;
        use crate::prelude::parser::memory_loader::MemoryIncludeLoader;
        use crate::prelude::parser::multi_loader::MultiIncludeLoader;
        use crate::prelude::parser::noop_loader::NoopIncludeLoader;

        let resolver = MultiIncludeLoader::<super::AsyncMultiIncludeLoaderItem>::default()
            .with_starts_with(
                "file://",
                Arc::new(MemoryIncludeLoader::from(vec![(
                    "file://basic.mjml",
                    "<mj-button>Hello</mj-button>",
                )])),
            )
            .with_any(Arc::<NoopIncludeLoader>::default());

        assert_eq!(
            resolver.resolve("file://basic.mjml").await.unwrap(),
            "<mj-button>Hello</mj-button>"
        );

        let err = resolver.resolve("file://not-found.mjml").await.unwrap_err();
        assert_eq!(err.reason, ErrorKind::NotFound);
        // assert_eq!(err.message.unwrap(), "unable to find compatible resolver");

        let err = resolver.resolve("noop://not-found.mjml").await.unwrap_err();
        assert_eq!(err.reason, ErrorKind::NotFound);
        assert!(err.message.is_none());
    }

    #[tokio::test]
    async fn should_not_find_resolver() {
        use crate::prelude::parser::loader::AsyncIncludeLoader;
        use crate::prelude::parser::multi_loader::MultiIncludeLoader;

        let resolver: MultiIncludeLoader<super::AsyncMultiIncludeLoaderItem> =
            MultiIncludeLoader::default();

        let err = resolver.resolve("file://not-found.mjml").await.unwrap_err();
        assert_eq!(err.reason, ErrorKind::NotFound);
        assert_eq!(err.message.unwrap(), "unable to find a compatible resolver");
    }
}
