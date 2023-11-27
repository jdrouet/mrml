use std::io::ErrorKind;
use std::sync::Arc;

use mrml::prelude::parser::loader::{IncludeLoader, IncludeLoaderError};

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize, tsify::Tsify)]
pub struct ReqwestIncludeLoaderOptions {
    pub base_url: String,
}

impl ReqwestIncludeLoaderOptions {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub fn build(self) -> Box<dyn IncludeLoader + Sync + Send + 'static> {
        Box::new(ReqwestIncludeLoader(Arc::new(ReqwestIncludeLoaderInner {
            base_url: self.base_url,
        })))
    }
}

#[derive(Debug)]
struct ReqwestIncludeLoaderInner {
    base_url: String,
}

impl ReqwestIncludeLoaderInner {
    async fn fetch(&self, path: String) -> Result<String, IncludeLoaderError> {
        reqwest::get(format!("{}{path}", self.base_url))
            .await
            .map_err(
                |err| IncludeLoaderError::new(path.clone(), ErrorKind::Other).with_cause(Arc::new(err))
            )?
            .text()
            .await
            .map_err(
                |err| IncludeLoaderError::new(path, ErrorKind::Other).with_cause(Arc::new(err))
            )
    }
}

#[derive(Debug)]
pub struct ReqwestIncludeLoader(Arc<ReqwestIncludeLoaderInner>);

#[async_trait::async_trait(?Send)]
impl IncludeLoader for ReqwestIncludeLoader {
    fn resolve(&self, path: &str) -> Result<String, IncludeLoaderError> {
        Err(IncludeLoaderError::new(path, ErrorKind::Other)
            .with_message("only compatible with async"))
    }

    async fn async_resolve(&self, path: &str) -> Result<String, IncludeLoaderError> {
        self.0.fetch(path.to_owned()).await
    }
}
