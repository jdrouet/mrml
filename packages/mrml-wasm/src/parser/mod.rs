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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, tsify::Tsify)]
#[serde(rename_all = "kebab-case", tag = "type")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum ParserError {
    UnexpectedElement {
        origin: super::Origin,
        position: super::Span,
    },
    UnexpectedToken {
        origin: super::Origin,
        position: super::Span,
    },
    MissingAttribute {
        name: String,
        origin: super::Origin,
        position: super::Span,
    },
    InvalidAttribute {
        origin: super::Origin,
        position: super::Span,
    },
    InvalidFormat {
        origin: super::Origin,
        position: super::Span,
    },
    EndOfStream {
        origin: super::Origin,
    },
    /// The input string should be smaller than 4GiB.
    SizeLimit {
        origin: super::Origin,
    },
    /// Errors detected by the `xmlparser` crate.
    ParserError {
        origin: super::Origin,
        source: String,
    },
    /// The Mjml document must have at least one element.
    NoRootNode,
    IncludeLoaderError {
        origin: super::Origin,
        position: super::Span,
        source: String,
    },
}

impl From<mrml::prelude::parser::Error> for ParserError {
    fn from(value: mrml::prelude::parser::Error) -> Self {
        use mrml::prelude::parser::Error;

        match value {
            Error::NoRootNode => Self::NoRootNode,
            Error::EndOfStream { origin } => Self::EndOfStream {
                origin: origin.into(),
            },
            Error::IncludeLoaderError {
                origin,
                position,
                source,
            } => Self::IncludeLoaderError {
                origin: origin.into(),
                position: position.into(),
                source: source.to_string(),
            },
            Error::InvalidAttribute { origin, position } => Self::InvalidAttribute {
                origin: origin.into(),
                position: position.into(),
            },
            Error::InvalidFormat { origin, position } => Self::InvalidFormat {
                origin: origin.into(),
                position: position.into(),
            },
            Error::MissingAttribute {
                name,
                origin,
                position,
            } => Self::MissingAttribute {
                name: name.into(),
                origin: origin.into(),
                position: position.into(),
            },
            Error::ParserError { origin, source } => Self::ParserError {
                origin: origin.into(),
                source: source.to_string(),
            },
            Error::SizeLimit { origin } => Self::SizeLimit {
                origin: origin.into(),
            },
            Error::UnexpectedElement { origin, position } => Self::UnexpectedElement {
                origin: origin.into(),
                position: position.into(),
            },
            Error::UnexpectedToken { origin, position } => Self::UnexpectedToken {
                origin: origin.into(),
                position: position.into(),
            },
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, tsify::Tsify)]
#[serde(rename_all = "kebab-case", tag = "type")]
#[tsify(into_wasm_abi)]
pub enum Origin {
    Root,
    Include { path: String },
}

impl From<mrml::prelude::parser::Origin> for Origin {
    fn from(value: mrml::prelude::parser::Origin) -> Self {
        match value {
            mrml::prelude::parser::Origin::Root => Self::Root,
            mrml::prelude::parser::Origin::Include { path } => Self::Include { path },
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, tsify::Tsify)]
#[tsify(into_wasm_abi)]
pub struct Span {
    start: usize,
    end: usize,
}

impl From<mrml::prelude::parser::Span> for Span {
    fn from(value: mrml::prelude::parser::Span) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, tsify::Tsify)]
#[serde(rename_all = "kebab-case")]
#[tsify(into_wasm_abi)]
pub enum WarningKind {
    UnexpectedAttributes,
}

impl From<mrml::prelude::parser::WarningKind> for WarningKind {
    fn from(value: mrml::prelude::parser::WarningKind) -> Self {
        match value {
            mrml::prelude::parser::WarningKind::UnexpectedAttribute => Self::UnexpectedAttributes,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, tsify::Tsify)]
#[tsify(into_wasm_abi)]
pub struct Warning {
    kind: WarningKind,
    origin: Origin,
    span: Span,
}

impl Warning {
    #[inline]
    pub(crate) fn from_vec(list: Vec<mrml::prelude::parser::Warning>) -> Vec<Warning> {
        list.into_iter().map(Warning::from).collect()
    }
}

impl From<mrml::prelude::parser::Warning> for Warning {
    fn from(value: mrml::prelude::parser::Warning) -> Self {
        Self {
            origin: value.origin.into(),
            kind: value.kind.into(),
            span: value.span.into(),
        }
    }
}
