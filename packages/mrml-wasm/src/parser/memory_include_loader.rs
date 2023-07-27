use std::collections::HashMap;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, tsify::Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct MemoryIncludeLoaderOptions {
    pub content: HashMap<String, String>,
}

impl MemoryIncludeLoaderOptions {
    pub fn build(self) -> Box<dyn mrml::prelude::parser::loader::IncludeLoader> {
        Box::new(mrml::prelude::parser::memory_loader::MemoryIncludeLoader::from(self.content))
    }
}
