//! Module containing a loader where all the possible files are stored on an
//! http server.

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::io::ErrorKind;
use std::sync::Arc;

use super::loader::IncludeLoaderError;
#[cfg(feature = "async")]
use crate::prelude::parser::loader::AsyncIncludeLoader;
use crate::prelude::parser::loader::IncludeLoader;

pub trait HttpFetcher: Default + Debug {
    fn fetch(
        &self,
        url: &str,
        headers: &HashMap<String, String>,
    ) -> Result<String, IncludeLoaderError>;
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
pub trait AsyncHttpFetcher: Default + Debug {
    async fn async_fetch(
        &self,
        url: &str,
        headers: &HashMap<String, String>,
    ) -> Result<String, IncludeLoaderError>;
}

#[cfg(feature = "http-loader-blocking-reqwest")]
#[derive(Debug, Default)]
pub struct BlockingReqwestFetcher(reqwest::blocking::Client);

#[cfg(feature = "http-loader-blocking-reqwest")]
impl HttpFetcher for BlockingReqwestFetcher {
    fn fetch(
        &self,
        url: &str,
        headers: &HashMap<String, String>,
    ) -> Result<String, IncludeLoaderError> {
        let req = self.0.get(url);
        let req = headers
            .iter()
            .fold(req, |r, (key, value)| r.header(key, value));
        let res = req.send().map_err(|err| {
            IncludeLoaderError::new(url, ErrorKind::NotFound)
                .with_message("unable to fetch template")
                .with_cause(Arc::new(err))
        })?;
        let res = res.error_for_status().map_err(|err| {
            IncludeLoaderError::new(url, ErrorKind::NotFound)
                .with_message("unable to fetch template")
                .with_cause(Arc::new(err))
        })?;
        res.text().map_err(|err| {
            IncludeLoaderError::new(url, ErrorKind::InvalidData)
                .with_message("unable to convert remote template as string")
                .with_cause(Arc::new(err))
        })
    }
}

#[cfg(feature = "http-loader-async-reqwest")]
#[derive(Debug, Default)]
pub struct AsyncReqwestFetcher(reqwest::Client);

#[cfg(feature = "http-loader-async-reqwest")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncHttpFetcher for AsyncReqwestFetcher {
    async fn async_fetch(
        &self,
        url: &str,
        headers: &HashMap<String, String>,
    ) -> Result<String, IncludeLoaderError> {
        let req = self.0.get(url);
        let req = headers
            .iter()
            .fold(req, |r, (key, value)| r.header(key, value));
        let res = req.send().await.map_err(|err| {
            IncludeLoaderError::new(url, ErrorKind::NotFound)
                .with_message("unable to fetch template")
                .with_cause(Arc::new(err))
        })?;
        let res = res.error_for_status().map_err(|err| {
            IncludeLoaderError::new(url, ErrorKind::NotFound)
                .with_message("unable to fetch template")
                .with_cause(Arc::new(err))
        })?;
        res.text().await.map_err(|err| {
            IncludeLoaderError::new(url, ErrorKind::InvalidData)
                .with_message("unable to convert remote template as string")
                .with_cause(Arc::new(err))
        })
    }
}

#[cfg(feature = "http-loader-ureq")]
#[derive(Debug, Default)]
pub struct UreqFetcher;

#[cfg(feature = "http-loader-ureq")]
impl HttpFetcher for UreqFetcher {
    fn fetch(
        &self,
        url: &str,
        headers: &HashMap<String, String>,
    ) -> Result<String, IncludeLoaderError> {
        let req = ureq::get(url);
        let req = headers
            .iter()
            .fold(req, |r, (key, value)| r.set(key.as_str(), value.as_str()));
        req.call()
            .map_err(|err| {
                IncludeLoaderError::new(url, ErrorKind::NotFound)
                    .with_message("unable to fetch template")
                    .with_cause(Arc::new(err))
            })?
            .into_string()
            .map_err(|err| {
                IncludeLoaderError::new(url, ErrorKind::InvalidData)
                    .with_message("unable to convert remote template as string")
                    .with_cause(Arc::new(err))
            })
    }
}

#[derive(Debug)]
/// This enum is a representation of the origin filtering strategy.
pub enum OriginList {
    Allow(HashSet<String>),
    Deny(HashSet<String>),
}

impl Default for OriginList {
    fn default() -> Self {
        // The default implementation will allow nothing, for security reasons.
        // If you need to allow everything, you'll have to specify it.
        Self::Allow(HashSet::new())
    }
}

impl OriginList {
    fn is_allowed(&self, origin: &str) -> bool {
        match self {
            Self::Allow(list) => list.contains(origin),
            Self::Deny(list) => !list.contains(origin),
        }
    }
}

#[derive(Debug, Default)]
/// This struct is an
/// [`IncludeLoader`](crate::prelude::parser::loader::IncludeLoader) where
/// you can read a template from an http server and be able to use it with
/// [`mj-include`](crate::mj_include).
///
/// # Example with `reqwest`
/// ```rust
/// #[cfg(feature = "http-loader-blocking-reqwest")]
/// {
///     use mrml::prelude::parser::http_loader::{HttpIncludeLoader, BlockingReqwestFetcher};
///     use mrml::prelude::parser::ParserOptions;
///     use std::collections::HashSet;
///
///     let resolver = HttpIncludeLoader::<BlockingReqwestFetcher>::new_allow(HashSet::from(["http://localhost".to_string()]));
///     let opts = ParserOptions {
///         include_loader: Box::new(resolver),
///     };
///     let template = r#"<mjml>
///       <mj-body>
///         <mj-include path="http://localhost/partials/mj-body.mjml" />
///       </mj-body>
///     </mjml>"#;
///     match mrml::parse_with_options(template, &opts) {
///         Ok(_) => println!("Success!"),
///         Err(err) => eprintln!("Couldn't parse template: {err:?}"),
///     }
/// }
/// ```
///
/// # Example with `ureq`
/// ```rust
/// #[cfg(feature = "http-loader-ureq")]
/// {
///     use mrml::prelude::parser::http_loader::{HttpIncludeLoader, UreqFetcher};
///     use mrml::prelude::parser::ParserOptions;
///     use std::collections::HashSet;
///
///     let resolver = HttpIncludeLoader::<UreqFetcher>::new_allow(HashSet::from(["http://localhost".to_string()]));
///     let opts = ParserOptions {
///         include_loader: Box::new(resolver),
///     };
///     let template = r#"<mjml>
///       <mj-body>
///         <mj-include path="http://localhost/partials/mj-body.mjml" />
///       </mj-body>
///     </mjml>"#;
///     match mrml::parse_with_options(template, &opts) {
///         Ok(_) => println!("Success!"),
///         Err(err) => eprintln!("Couldn't parse template: {err:?}"),
///     }
/// }
/// ```
pub struct HttpIncludeLoader<F> {
    origin: OriginList,
    headers: HashMap<String, String>,
    fetcher: F,
}

impl<F: Default> HttpIncludeLoader<F> {
    /// Creates a new
    /// [`HttpIncludeLoader`](crate::prelude::parser::http_loader::HttpIncludeLoader)
    /// that allows all the origins.
    ///
    /// If you use this method, you should be careful, you could be loading some
    /// data from anywhere.
    pub fn allow_all() -> Self {
        Self {
            origin: OriginList::Deny(Default::default()),
            headers: HashMap::default(),
            fetcher: F::default(),
        }
    }

    /// Creates a new instance with an allow list to filter the origins.
    ///
    /// # Example with `reqwest`
    /// ```rust
    /// #[cfg(feature = "http-loader-blocking-reqwest")]
    /// {
    ///     use mrml::prelude::parser::http_loader::{HttpIncludeLoader, BlockingReqwestFetcher};
    ///     use std::collections::HashSet;
    ///
    ///     let resolver = HttpIncludeLoader::<BlockingReqwestFetcher>::new_allow(HashSet::from(["http://localhost".to_string()]));
    /// }
    /// ```
    ///
    /// # Example with `ureq`
    /// ```
    /// #[cfg(feature = "http-loader-ureq")]
    /// {
    ///     use mrml::prelude::parser::http_loader::{HttpIncludeLoader, UreqFetcher};
    ///     use std::collections::HashSet;
    ///
    ///     let resolver = HttpIncludeLoader::<UreqFetcher>::new_allow(HashSet::from(["http://localhost".to_string()]));
    /// }
    /// ```
    pub fn new_allow(origins: HashSet<String>) -> Self {
        Self {
            origin: OriginList::Allow(origins),
            headers: HashMap::default(),
            fetcher: F::default(),
        }
    }

    /// Creates a new instance with an dey list to filter the origins.
    ///
    /// # Example with `reqwest`
    /// ```rust
    /// #[cfg(feature = "http-loader-blocking-reqwest")]
    /// {
    ///     use mrml::prelude::parser::http_loader::{HttpIncludeLoader, BlockingReqwestFetcher};
    ///     use std::collections::HashSet;
    ///
    ///     let resolver = HttpIncludeLoader::<BlockingReqwestFetcher>::new_allow(HashSet::from(["http://somewhere.com".to_string()]));
    /// }
    /// ```
    ///
    /// # Example with `ureq`
    /// ```rust
    /// #[cfg(feature = "http-loader-ureq")]
    /// {
    ///     use mrml::prelude::parser::http_loader::{HttpIncludeLoader, UreqFetcher};
    ///     use std::collections::HashSet;
    ///
    ///     let resolver = HttpIncludeLoader::<UreqFetcher>::new_allow(HashSet::from(["http://somewhere.com".to_string()]));
    /// }
    /// ```
    pub fn new_deny(origins: HashSet<String>) -> Self {
        Self {
            origin: OriginList::Deny(origins),
            headers: HashMap::default(),
            fetcher: F::default(),
        }
    }

    pub fn with_header<K: ToString, V: ToString>(mut self, name: K, value: V) -> Self {
        self.headers.insert(name.to_string(), value.to_string());
        self
    }

    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = headers;
        self
    }

    pub fn set_header<K: ToString, V: ToString>(&mut self, name: K, value: V) {
        self.headers.insert(name.to_string(), value.to_string());
    }

    pub fn set_headers(&mut self, headers: HashMap<String, String>) {
        self.headers = headers;
    }

    /// Check that the given url provided by the `path` attribute in the
    /// `mj-include` complies with the filtering.
    fn check_url(&self, path: &str) -> Result<(), IncludeLoaderError> {
        let url = url::Url::parse(path).map_err(|err| {
            IncludeLoaderError::new(path, ErrorKind::InvalidInput)
                .with_message("unable to parse the provided url")
                .with_cause(Arc::new(err))
        })?;
        let origin = url.origin().ascii_serialization();
        if self.origin.is_allowed(&origin) {
            Ok(())
        } else {
            Err(IncludeLoaderError::new(path, ErrorKind::InvalidInput)
                .with_message("the path is not allowed by the defined list of domains"))
        }
    }
}

impl<F: HttpFetcher> IncludeLoader for HttpIncludeLoader<F> {
    fn resolve(&self, path: &str) -> Result<String, IncludeLoaderError> {
        self.check_url(path)?;
        self.fetcher.fetch(path, &self.headers)
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl<F: AsyncHttpFetcher + Sync + Send> AsyncIncludeLoader for HttpIncludeLoader<F> {
    async fn async_resolve(&self, path: &str) -> Result<String, IncludeLoaderError> {
        self.check_url(path)?;
        self.fetcher.async_fetch(path, &self.headers).await
    }
}

#[cfg(test)]
mod common_tests {
    use std::collections::HashSet;

    use super::OriginList;

    #[test]
    fn origin_list_is_allowed() {
        assert!(!OriginList::Allow(Default::default()).is_allowed("localhost"));
        assert!(OriginList::Allow(HashSet::from(["localhost".to_string()])).is_allowed("localhost"));
        assert!(OriginList::Deny(HashSet::from(["somewhere".to_string()])).is_allowed("localhost"));
        assert!(!OriginList::Deny(HashSet::from(["somewhere".to_string()])).is_allowed("somewhere"));
        assert!(OriginList::Deny(HashSet::default()).is_allowed("somewhere"));
    }
}

#[cfg(all(test, feature = "http-loader-ureq"))]
mod ureq_tests {
    use std::collections::{HashMap, HashSet};
    use std::io::ErrorKind;

    use super::{HttpIncludeLoader, UreqFetcher};
    use crate::prelude::parser::loader::IncludeLoader;

    #[test]
    fn include_loader_should_implement_debug() {
        let _ = format!("{:?}", HttpIncludeLoader::<UreqFetcher>::default());
    }

    #[test]
    fn include_loader_should_validate_url() {
        // allow everything
        assert!(HttpIncludeLoader::<UreqFetcher>::allow_all()
            .check_url("http://localhost/partial.mjml")
            .is_ok());
        // allow nothing
        assert!(
            HttpIncludeLoader::<UreqFetcher>::new_allow(HashSet::default())
                .check_url("http://localhost/partial.mjml")
                .is_err()
        );
        assert!(HttpIncludeLoader::<UreqFetcher>::default()
            .check_url("http://localhost/partial.mjml")
            .is_err());
        // only deny some domains
        let loader = HttpIncludeLoader::<UreqFetcher>::new_deny(HashSet::from([
            "http://somewhere".to_string(),
        ]));
        assert!(loader.check_url("http://localhost/partial.mjml").is_ok());
        assert!(loader.check_url("http://somewhere/partial.mjml").is_err());
        assert!(loader.check_url("https://somewhere/partial.mjml").is_ok());
        // only allow some domains
        let loader = HttpIncludeLoader::<UreqFetcher>::new_allow(HashSet::from([
            "http://localhost".to_string(),
            "https://somewhere".to_string(),
        ]));
        assert!(loader.check_url("http://localhost/partial.mjml").is_ok());
        assert!(loader.check_url("http://somewhere/partial.mjml").is_err());
        assert!(loader.check_url("https://somewhere/partial.mjml").is_ok());
        // invalid urls
        assert_eq!(
            loader.check_url("").unwrap_err().message.unwrap(),
            "unable to parse the provided url"
        );
    }

    #[test]
    fn include_loader_should_resolve_with_content() {
        let partial = "<mj-text>Hello World!</mj-text>";
        let mut mock_server = mockito::Server::new();
        let m = mock_server
            .mock("GET", "/partial.mjml")
            .with_status(200)
            .with_body("<mj-text>Hello World!</mj-text>")
            .create();
        let mut loader =
            HttpIncludeLoader::<UreqFetcher>::new_allow(HashSet::from([mock_server.url()]));
        loader.set_header("foo", "bar");
        loader.set_headers(Default::default());
        let resolved = loader
            .resolve(&format!("{}/partial.mjml", mock_server.url()))
            .unwrap();
        assert_eq!(partial, resolved);
        m.assert();
    }

    #[test]
    fn include_loader_should_resolve_with_not_found() {
        let mut mock_server = mockito::Server::new();
        let m = mock_server
            .mock("GET", "/partial.mjml")
            .with_status(404)
            .with_body("Not Found")
            .create();
        let loader =
            HttpIncludeLoader::<UreqFetcher>::new_allow(HashSet::from([mock_server.url()]));
        let err = loader
            .resolve(&format!("{}/partial.mjml", mock_server.url()))
            .unwrap_err();
        assert_eq!(err.reason, ErrorKind::NotFound);
        m.assert();
    }

    #[test]
    fn include_loader_should_resolve_with_headers() {
        let mut mock_server = mockito::Server::new();
        let m = mock_server
            .mock("GET", "/partial.mjml")
            .match_header("user-agent", "mrml-test")
            .with_status(404)
            .with_body("Not Found")
            .create();
        let loader =
            HttpIncludeLoader::<UreqFetcher>::new_allow(HashSet::from([mock_server.url()]))
                .with_header("user-agent", "invalid")
                .with_headers(HashMap::from([(
                    "user-agent".to_string(),
                    "mrml-test".to_string(),
                )]));
        let err = loader
            .resolve(&format!("{}/partial.mjml", mock_server.url()))
            .unwrap_err();
        assert_eq!(err.reason, ErrorKind::NotFound);
        m.assert();
    }
}

#[cfg(all(test, feature = "http-loader-blocking-reqwest"))]
mod reqwest_tests {
    use std::collections::{HashMap, HashSet};
    use std::io::ErrorKind;

    use super::{BlockingReqwestFetcher, HttpIncludeLoader};
    use crate::prelude::parser::loader::IncludeLoader;

    #[test]
    fn include_loader_should_implement_debug() {
        let _ = format!(
            "{:?}",
            HttpIncludeLoader::<BlockingReqwestFetcher>::default()
        );
    }

    #[test]
    fn include_loader_should_validate_url() {
        // allow everything
        assert!(HttpIncludeLoader::<BlockingReqwestFetcher>::allow_all()
            .check_url("http://localhost/partial.mjml")
            .is_ok());
        // allow nothing
        assert!(
            HttpIncludeLoader::<BlockingReqwestFetcher>::new_allow(HashSet::default())
                .check_url("http://localhost/partial.mjml")
                .is_err()
        );
        assert!(HttpIncludeLoader::<BlockingReqwestFetcher>::default()
            .check_url("http://localhost/partial.mjml")
            .is_err());
        // only deny some domains
        let loader = HttpIncludeLoader::<BlockingReqwestFetcher>::new_deny(HashSet::from([
            "http://somewhere".to_string(),
        ]));
        assert!(loader.check_url("http://localhost/partial.mjml").is_ok());
        assert!(loader.check_url("http://somewhere/partial.mjml").is_err());
        assert!(loader.check_url("https://somewhere/partial.mjml").is_ok());
        // only allow some domains
        let loader = HttpIncludeLoader::<BlockingReqwestFetcher>::new_allow(HashSet::from([
            "http://localhost".to_string(),
            "https://somewhere".to_string(),
        ]));
        assert!(loader.check_url("http://localhost/partial.mjml").is_ok());
        assert!(loader.check_url("http://somewhere/partial.mjml").is_err());
        assert!(loader.check_url("https://somewhere/partial.mjml").is_ok());
        // invalid urls
        assert_eq!(
            loader.check_url("").unwrap_err().message.unwrap(),
            "unable to parse the provided url"
        );
    }

    #[test]
    fn include_loader_should_resolve_with_content() {
        let partial = "<mj-text>Hello World!</mj-text>";
        let mut mock_server = mockito::Server::new();
        let m = mock_server
            .mock("GET", "/partial.mjml")
            .with_status(200)
            .with_body("<mj-text>Hello World!</mj-text>")
            .create();
        let mut loader = HttpIncludeLoader::<BlockingReqwestFetcher>::new_allow(HashSet::from([
            mock_server.url(),
        ]));
        loader.set_header("foo", "bar");
        loader.set_headers(Default::default());
        let resolved = loader
            .resolve(&format!("{}/partial.mjml", mock_server.url()))
            .unwrap();
        assert_eq!(partial, resolved);
        m.assert();
    }

    #[test]
    fn include_loader_should_resolve_with_not_found() {
        let mut mock_server = mockito::Server::new();
        let m = mock_server
            .mock("GET", "/partial.mjml")
            .with_status(404)
            .with_body("Not Found")
            .create();
        let loader = HttpIncludeLoader::<BlockingReqwestFetcher>::new_allow(HashSet::from([
            mock_server.url(),
        ]));
        let err = loader
            .resolve(&format!("{}/partial.mjml", mock_server.url()))
            .unwrap_err();
        assert_eq!(err.reason, ErrorKind::NotFound);
        m.assert();
    }

    #[test]
    fn include_loader_should_resolve_with_headers() {
        let mut mock_server = mockito::Server::new();
        let m = mock_server
            .mock("GET", "/partial.mjml")
            .match_header("user-agent", "mrml-test")
            .with_status(404)
            .with_body("Not Found")
            .create();
        let loader = HttpIncludeLoader::<BlockingReqwestFetcher>::new_allow(HashSet::from([
            mock_server.url(),
        ]))
        .with_header("user-agent", "invalid")
        .with_headers(HashMap::from([(
            "user-agent".to_string(),
            "mrml-test".to_string(),
        )]));
        let err = loader
            .resolve(&format!("{}/partial.mjml", mock_server.url()))
            .unwrap_err();
        assert_eq!(err.reason, ErrorKind::NotFound);
        m.assert();
    }
}
