mod memory_include_loader;
#[cfg(feature = "reqwest-include-loader")]
mod reqwest_include_loader;

pub use memory_include_loader::*;
#[cfg(feature = "reqwest-include-loader")]
pub use reqwest_include_loader::*;

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
    pub fn build(
        self,
    ) -> Box<dyn mrml::prelude::parser::loader::IncludeLoader + Send + Sync + 'static> {
        match self {
            Self::Noop => Box::new(mrml::prelude::parser::noop_loader::NoopIncludeLoader),
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

impl From<ParserOptions> for mrml::prelude::parser::ParserOptions {
    fn from(value: ParserOptions) -> Self {
        mrml::prelude::parser::ParserOptions {
            include_loader: value.include_loader.build(),
        }
    }
}

// ASYNC RELATED
#[cfg(feature = "async")]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, tsify::Tsify)]
#[serde(tag = "type", rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum AsyncIncludeLoaderOptions {
    Noop,
    Memory(MemoryIncludeLoaderOptions),
    #[cfg(feature = "reqwest-include-loader")]
    Reqwest(ReqwestIncludeLoaderOptions),
}

#[cfg(feature = "async")]
impl Default for AsyncIncludeLoaderOptions {
    fn default() -> Self {
        Self::Noop
    }
}

#[cfg(feature = "async")]
impl AsyncIncludeLoaderOptions {
    pub fn build_async(
        self,
    ) -> Box<dyn mrml::prelude::parser::loader::AsyncIncludeLoader + Send + Sync + 'static> {
        match self {
            Self::Noop => Box::new(mrml::prelude::parser::noop_loader::NoopIncludeLoader),
            Self::Memory(inner) => inner.build_async(),
            #[cfg(feature = "reqwest-include-loader")]
            Self::Reqwest(inner) => inner.build_async(),
        }
    }
}

#[cfg(feature = "async")]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, tsify::Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct AsyncParserOptions {
    pub include_loader: AsyncIncludeLoaderOptions,
}

#[cfg(feature = "async")]
impl From<AsyncParserOptions> for mrml::prelude::parser::AsyncParserOptions {
    fn from(value: AsyncParserOptions) -> Self {
        mrml::prelude::parser::AsyncParserOptions {
            include_loader: value.include_loader.build_async(),
        }
    }
}
