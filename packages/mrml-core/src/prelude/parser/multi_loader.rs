//! Module containing a loader that is composed of multiple loaders.

use super::loader::IncludeLoaderError;
#[cfg(feature = "async")]
use crate::prelude::parser::loader::AsyncIncludeLoader;
use crate::prelude::parser::loader::IncludeLoader;

#[derive(Debug, Default)]
/// This struct is a [`IncludeLoader`] where you can define a strategy of
/// resolver depending on the path.
/// That way, you can have a resolver for paths starting with `https://` and
/// another resolver for local files where the paths start with `file://`.
/// If no provider match the path, a `NotFound` error will be returned.
///
/// # Example
/// ```rust
/// use mrml::mj_include::body::MjIncludeBodyKind;
/// use mrml::prelude::parser::memory_loader::MemoryIncludeLoader;
/// use mrml::prelude::parser::multi_loader::{MultiIncludeLoader, MultiIncludeLoaderItem, MultiIncludeLoaderFilter};
/// use mrml::prelude::parser::noop_loader::NoopIncludeLoader;
/// use mrml::prelude::parser::loader::IncludeLoader;
/// use mrml::prelude::parser::ParserOptions;
///
/// let resolver = MultiIncludeLoader::<Box<dyn IncludeLoader + 'static>>::new()
///     .with_starts_with(
///         "file://",
///         Box::new(MemoryIncludeLoader::from(vec![(
///             "file://basic.mjml",
///             "<mj-button>Hello</mj-button>",
///         )])),
///     )
///     .with_any(Box::<NoopIncludeLoader>::default());
/// let opts = ParserOptions {
///     include_loader: Box::new(resolver),
/// };
/// let json = r#"<mjml>
///   <mj-body>
///     <mj-include path="file://basic.mjml" />
///   </mj-body>
/// </mjml>"#;
/// match mrml::parse_with_options(json, &opts) {
///     Ok(_) => println!("Success!"),
///     Err(err) => eprintln!("Couldn't parse template: {err:?}"),
/// }
/// ```
///
/// # Example async
///
/// ```rust
/// #[cfg(feature = "http-loader-async-reqwest")]
/// # tokio_test::block_on(async {
/// use mrml::mj_include::body::MjIncludeBodyKind;
/// use mrml::prelude::parser::http_loader::{AsyncReqwestFetcher, HttpIncludeLoader};
/// use mrml::prelude::parser::memory_loader::MemoryIncludeLoader;
/// use mrml::prelude::parser::multi_loader::{MultiIncludeLoader, MultiIncludeLoaderItem, MultiIncludeLoaderFilter};
/// use mrml::prelude::parser::noop_loader::NoopIncludeLoader;
/// use mrml::prelude::parser::loader::AsyncIncludeLoader;
/// use mrml::prelude::parser::AsyncParserOptions;
///
/// let resolver = MultiIncludeLoader::<Box<dyn AsyncIncludeLoader + Send + Sync + 'static>>::new()
///     .with_starts_with("https://", Box::new(HttpIncludeLoader::<AsyncReqwestFetcher>::allow_all()))
///     .with_any(Box::<NoopIncludeLoader>::default());
/// let opts = AsyncParserOptions {
///     include_loader: Box::new(resolver),
/// };
/// let json = r#"<mjml>
///   <mj-body>
///     <mj-include path="file://basic.mjml" />
///   </mj-body>
/// </mjml>"#;
/// match mrml::async_parse_with_options(json, std::sync::Arc::new(opts)).await {
///     Ok(_) => println!("Success!"),
///     Err(err) => eprintln!("Couldn't parse template: {err:?}"),
/// }
/// # })
/// ```
pub struct MultiIncludeLoader<T>(Vec<MultiIncludeLoaderItem<T>>);

impl<T> MultiIncludeLoader<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    fn with_item(mut self, filter: MultiIncludeLoaderFilter, loader: T) -> Self {
        self.0.push(MultiIncludeLoaderItem { filter, loader });
        self
    }

    #[inline]
    pub fn with_any(self, loader: T) -> Self {
        self.with_item(MultiIncludeLoaderFilter::Any, loader)
    }

    #[inline]
    pub fn with_starts_with<S: ToString>(self, starts_with: S, loader: T) -> Self {
        self.with_item(
            MultiIncludeLoaderFilter::StartsWith {
                value: starts_with.to_string(),
            },
            loader,
        )
    }

    fn add_item(&mut self, filter: MultiIncludeLoaderFilter, loader: T) {
        self.0.push(MultiIncludeLoaderItem { filter, loader });
    }

    #[inline]
    pub fn add_any(&mut self, loader: T) {
        self.add_item(MultiIncludeLoaderFilter::Any, loader);
    }

    #[inline]
    pub fn add_starts_with<S: ToString>(&mut self, starts_with: S, loader: T) {
        self.add_item(
            MultiIncludeLoaderFilter::StartsWith {
                value: starts_with.to_string(),
            },
            loader,
        );
    }
}

#[derive(Debug)]
pub enum MultiIncludeLoaderFilter {
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
pub struct MultiIncludeLoaderItem<T> {
    pub filter: MultiIncludeLoaderFilter,
    pub loader: T,
}

pub type MultiIncludeLoaderSync = MultiIncludeLoader<Box<dyn IncludeLoader + 'static>>;

impl IncludeLoader for MultiIncludeLoaderSync {
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
}

#[cfg(feature = "async")]
pub type MultiIncludeLoaderAsync =
    MultiIncludeLoader<Box<dyn AsyncIncludeLoader + Sync + Send + 'static>>;

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncIncludeLoader for MultiIncludeLoaderAsync {
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

    use super::{MultiIncludeLoaderFilter, MultiIncludeLoaderItem};

    #[test]
    fn should_resolve_sync() {
        use crate::prelude::parser::loader::IncludeLoader;
        use crate::prelude::parser::memory_loader::MemoryIncludeLoader;
        use crate::prelude::parser::multi_loader::MultiIncludeLoader;
        use crate::prelude::parser::noop_loader::NoopIncludeLoader;

        let resolver: MultiIncludeLoader<Box<dyn IncludeLoader>> = MultiIncludeLoader(vec![
            MultiIncludeLoaderItem {
                filter: MultiIncludeLoaderFilter::StartsWith {
                    value: "file://".into(),
                },
                loader: Box::new(MemoryIncludeLoader::from(vec![(
                    "file://basic.mjml",
                    "<mj-button>Hello</mj-button>",
                )])),
            },
            MultiIncludeLoaderItem {
                filter: MultiIncludeLoaderFilter::Any,
                loader: Box::<NoopIncludeLoader>::default(),
            },
        ]);
        assert_eq!(
            resolver.resolve("file://basic.mjml").unwrap(),
            "<mj-button>Hello</mj-button>"
        );

        let err = resolver.resolve("file://not-found.mjml").unwrap_err();
        assert_eq!(err.reason, ErrorKind::NotFound);

        let err = resolver.resolve("noop://not-found.mjml").unwrap_err();
        assert_eq!(err.reason, ErrorKind::NotFound);
        assert!(err.message.is_none());
    }

    #[test]
    fn should_not_find_resolver_sync() {
        use crate::prelude::parser::loader::IncludeLoader;
        use crate::prelude::parser::multi_loader::MultiIncludeLoader;

        let resolver = MultiIncludeLoader(vec![]);

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

#[cfg(all(test, feature = "async"))]
mod async_tests {
    use std::io::ErrorKind;

    #[tokio::test]
    async fn should_resolve_async() {
        use crate::prelude::parser::loader::AsyncIncludeLoader;
        use crate::prelude::parser::memory_loader::MemoryIncludeLoader;
        use crate::prelude::parser::multi_loader::MultiIncludeLoader;
        use crate::prelude::parser::noop_loader::NoopIncludeLoader;

        let resolver =
            MultiIncludeLoader::<Box<dyn AsyncIncludeLoader + Sync + Send + 'static>>::new()
                .with_starts_with(
                    "file://",
                    Box::new(MemoryIncludeLoader::from(vec![(
                        "file://basic.mjml",
                        "<mj-button>Hello</mj-button>",
                    )])),
                )
                .with_any(Box::<NoopIncludeLoader>::default());

        assert_eq!(
            resolver.async_resolve("file://basic.mjml").await.unwrap(),
            "<mj-button>Hello</mj-button>"
        );

        let err = resolver
            .async_resolve("file://not-found.mjml")
            .await
            .unwrap_err();
        assert_eq!(err.reason, ErrorKind::NotFound);

        let err = resolver
            .async_resolve("noop://not-found.mjml")
            .await
            .unwrap_err();
        assert_eq!(err.reason, ErrorKind::NotFound);
        assert!(err.message.is_none());
    }

    #[tokio::test]
    async fn should_not_find_resolver_async() {
        use crate::prelude::parser::loader::AsyncIncludeLoader;
        use crate::prelude::parser::multi_loader::MultiIncludeLoader;

        let resolver = MultiIncludeLoader::new();

        let err = resolver
            .async_resolve("file://not-found.mjml")
            .await
            .unwrap_err();
        assert_eq!(err.reason, ErrorKind::NotFound);
        assert_eq!(err.message.unwrap(), "unable to find a compatible resolver");
    }
}
