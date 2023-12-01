use std::collections::HashMap;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, tsify::Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct MemoryIncludeLoaderOptions {
    pub content: HashMap<String, String>,
}

impl MemoryIncludeLoaderOptions {
    pub fn build(self) -> Box<dyn mrml::prelude::parser::loader::IncludeLoader + 'static> {
        Box::new(mrml::prelude::parser::memory_loader::MemoryIncludeLoader::from(self.content))
    }

    #[cfg(feature = "async")]
    pub fn build_async(
        self,
    ) -> Box<dyn mrml::prelude::parser::loader::AsyncIncludeLoader + Send + Sync + 'static> {
        Box::new(mrml::prelude::parser::memory_loader::MemoryIncludeLoader::from(self.content))
    }
}
