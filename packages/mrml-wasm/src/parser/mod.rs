mod memory_include_loader;

pub use memory_include_loader::*;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, tsify::Tsify)]
#[serde(tag = "type", rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum IncludeLoaderOptions {
    Noop,
    Memory(MemoryIncludeLoaderOptions),
}

impl Default for IncludeLoaderOptions {
    fn default() -> Self {
        Self::Noop
    }
}

impl IncludeLoaderOptions {
    pub fn build(self) -> Box<dyn mrml::prelude::parse::loader::IncludeLoader> {
        match self {
            Self::Noop => Box::new(mrml::prelude::parse::noop_loader::NoopIncludeLoader),
            Self::Memory(inner) => inner.build(),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, tsify::Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ParserOptions {
    pub include_loader: IncludeLoaderOptions,
}

impl From<ParserOptions> for mrml::prelude::parse::ParserOptions {
    fn from(value: ParserOptions) -> Self {
        mrml::prelude::parse::ParserOptions {
            include_loader: value.include_loader.build(),
        }
    }
}
