use std::collections::HashMap;
use std::sync::Arc;

use mrml::prelude::parser::http_loader::{AsyncReqwestFetcher, HttpIncludeLoader};
use mrml::prelude::parser::loader::AsyncIncludeLoader;

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize, tsify::Tsify)]
pub struct ReqwestIncludeLoaderOptions {
    pub headers: HashMap<String, String>,
}

impl ReqwestIncludeLoaderOptions {
    pub fn new(headers: HashMap<String, String>) -> Self {
        Self { headers }
    }

    pub fn build_async(self) -> Arc<dyn AsyncIncludeLoader + Sync + Send + 'static> {
        Arc::new(HttpIncludeLoader::<AsyncReqwestFetcher>::allow_all().with_headers(self.headers))
    }
}
