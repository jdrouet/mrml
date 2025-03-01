#![allow(clippy::useless_conversion)]

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use mrml::prelude::parser::http_loader::{HttpIncludeLoader, UreqFetcher};
use mrml::prelude::parser::loader::IncludeLoader;
use mrml::prelude::parser::local_loader::LocalIncludeLoader;
use mrml::prelude::parser::memory_loader::MemoryIncludeLoader;
use mrml::prelude::parser::noop_loader::NoopIncludeLoader;
use pyo3::exceptions::PyIOError;
use pyo3::prelude::*;

#[pyclass(frozen)]
#[derive(Clone, Debug, Default)]
pub struct NoopIncludeLoaderOptions;

#[pyclass(frozen)]
#[derive(Clone, Debug, Default)]
pub struct MemoryIncludeLoaderOptions(HashMap<String, String>);

#[pyclass(frozen)]
#[derive(Clone, Debug, Default)]
pub struct LocalIncludeLoaderOptions(PathBuf);

#[pyclass(frozen, eq, eq_int)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HttpIncludeLoaderOptionsMode {
    Allow,
    Deny,
}

impl Default for HttpIncludeLoaderOptionsMode {
    fn default() -> Self {
        Self::Allow
    }
}

#[pyclass(frozen)]
#[derive(Clone, Debug, Default)]
pub struct HttpIncludeLoaderOptions {
    mode: HttpIncludeLoaderOptionsMode,
    list: HashSet<String>,
}

#[pyclass(frozen)]
#[derive(Clone, Debug)]
pub enum ParserIncludeLoaderOptions {
    Noop(NoopIncludeLoaderOptions),
    Memory(MemoryIncludeLoaderOptions),
    Local(LocalIncludeLoaderOptions),
    Http(HttpIncludeLoaderOptions),
}

impl Default for ParserIncludeLoaderOptions {
    fn default() -> Self {
        Self::Noop(NoopIncludeLoaderOptions)
    }
}

impl ParserIncludeLoaderOptions {
    fn build(self) -> Box<dyn IncludeLoader + Send + Sync> {
        match self {
            Self::Noop(_) => Box::<NoopIncludeLoader>::default(),
            Self::Memory(MemoryIncludeLoaderOptions(inner)) => {
                Box::new(MemoryIncludeLoader::from(inner))
            }
            Self::Local(LocalIncludeLoaderOptions(inner)) => {
                Box::new(LocalIncludeLoader::new(inner))
            }
            Self::Http(HttpIncludeLoaderOptions { mode, list }) => match mode {
                HttpIncludeLoaderOptionsMode::Allow => {
                    Box::new(HttpIncludeLoader::<UreqFetcher>::new_allow(list))
                }
                HttpIncludeLoaderOptionsMode::Deny => {
                    Box::new(HttpIncludeLoader::<UreqFetcher>::new_deny(list))
                }
            },
        }
    }
}

#[pyfunction]
#[pyo3(name = "noop_loader")]
pub fn noop_loader() -> ParserIncludeLoaderOptions {
    ParserIncludeLoaderOptions::Noop(NoopIncludeLoaderOptions)
}

#[pyfunction]
#[pyo3(name = "memory_loader", signature = (data = None))]
pub fn memory_loader(data: Option<HashMap<String, String>>) -> ParserIncludeLoaderOptions {
    ParserIncludeLoaderOptions::Memory(MemoryIncludeLoaderOptions(data.unwrap_or_default()))
}

#[pyfunction]
#[pyo3(name = "local_loader", signature = (data = None))]
pub fn local_loader(data: Option<String>) -> PyResult<ParserIncludeLoaderOptions> {
    let path = match data.map(PathBuf::from) {
        Some(path) if path.is_absolute() => path,
        Some(path) => std::env::current_dir()
            .map_err(|err| PyErr::new::<pyo3::exceptions::PyException, String>(err.to_string()))?
            .join(path),
        None => std::env::current_dir()
            .map_err(|err| PyErr::new::<pyo3::exceptions::PyException, String>(err.to_string()))?,
    };
    Ok(ParserIncludeLoaderOptions::Local(
        LocalIncludeLoaderOptions(path),
    ))
}

#[pyfunction]
#[pyo3(name = "http_loader", signature = (mode = None, list = None))]
pub fn http_loader(
    mode: Option<HttpIncludeLoaderOptionsMode>,
    list: Option<HashSet<String>>,
) -> ParserIncludeLoaderOptions {
    ParserIncludeLoaderOptions::Http(HttpIncludeLoaderOptions {
        mode: mode.unwrap_or(HttpIncludeLoaderOptionsMode::Allow),
        list: list.unwrap_or_default(),
    })
}

#[pyclass(frozen)]
#[derive(Clone, Debug, Default)]
pub struct ParserOptions {
    #[pyo3(get)]
    pub include_loader: ParserIncludeLoaderOptions,
}

#[pymethods]
impl ParserOptions {
    #[new]
    #[pyo3(signature = (include_loader=None))]
    pub fn new(include_loader: Option<ParserIncludeLoaderOptions>) -> Self {
        Self {
            include_loader: include_loader.unwrap_or_default(),
        }
    }
}

impl From<ParserOptions> for mrml::prelude::parser::ParserOptions {
    fn from(value: ParserOptions) -> Self {
        let include_loader = value.include_loader.build();
        mrml::prelude::parser::ParserOptions { include_loader }
    }
}

#[pyclass(frozen)]
#[derive(Clone, Debug, Default)]
pub struct RenderOptions {
    #[pyo3(get)]
    pub disable_comments: bool,
    #[pyo3(get)]
    pub social_icon_origin: Option<String>,
    #[pyo3(get)]
    pub fonts: Option<HashMap<String, String>>,
}

#[pymethods]
impl RenderOptions {
    #[new]
    #[pyo3(signature = (disable_comments=false, social_icon_origin=None, fonts=None))]
    pub fn new(
        disable_comments: bool,
        social_icon_origin: Option<String>,
        fonts: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            disable_comments,
            social_icon_origin,
            fonts,
        }
    }
}

impl From<RenderOptions> for mrml::prelude::render::RenderOptions {
    fn from(value: RenderOptions) -> Self {
        let mut opts = mrml::prelude::render::RenderOptions {
            disable_comments: value.disable_comments,
            ..Default::default()
        };
        if let Some(social) = value.social_icon_origin {
            opts.social_icon_origin = Some(Cow::Owned(social));
        }
        if let Some(fonts) = value.fonts {
            opts.fonts = fonts
                .into_iter()
                .map(|(name, value)| (name, Cow::Owned(value)))
                .collect();
        }
        opts
    }
}

#[pyclass(frozen)]
#[derive(Clone, Debug, Default)]
pub struct Warning {
    #[pyo3(get)]
    pub origin: Option<String>,
    #[pyo3(get)]
    pub kind: &'static str,
    #[pyo3(get)]
    pub start: usize,
    #[pyo3(get)]
    pub end: usize,
}

impl Warning {
    fn from_vec(input: Vec<mrml::prelude::parser::Warning>) -> Vec<Self> {
        input.into_iter().map(Self::from).collect()
    }
}

impl From<mrml::prelude::parser::Warning> for Warning {
    fn from(value: mrml::prelude::parser::Warning) -> Self {
        Self {
            origin: match value.origin {
                mrml::prelude::parser::Origin::Root => None,
                mrml::prelude::parser::Origin::Include { path } => Some(path),
            },
            kind: value.kind.as_str(),
            start: value.span.start,
            end: value.span.end,
        }
    }
}

#[pyclass(frozen)]
#[derive(Clone, Debug, Default)]
pub struct Output {
    #[pyo3(get)]
    pub content: String,
    #[pyo3(get)]
    pub warnings: Vec<Warning>,
}

#[pyfunction]
#[pyo3(name = "to_html", signature = (input, parser_options=None, render_options=None))]
fn to_html(
    input: String,
    parser_options: Option<ParserOptions>,
    render_options: Option<RenderOptions>,
) -> PyResult<Output> {
    let parser_options = parser_options.unwrap_or_default().into();
    let parsed = mrml::parse_with_options(input, &parser_options)
        .map_err(|err| PyIOError::new_err(err.to_string()))?;

    let render_options = render_options.unwrap_or_default().into();
    let content = parsed
        .element
        .render(&render_options)
        .map_err(|err| PyIOError::new_err(err.to_string()))?;

    Ok(Output {
        content,
        warnings: Warning::from_vec(parsed.warnings),
    })
}

#[pymodule]
#[pyo3(name = "mrml")]
fn register(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<NoopIncludeLoaderOptions>()?;
    m.add_class::<MemoryIncludeLoaderOptions>()?;
    m.add_class::<LocalIncludeLoaderOptions>()?;
    m.add_class::<HttpIncludeLoaderOptions>()?;
    m.add_class::<HttpIncludeLoaderOptionsMode>()?;
    m.add_class::<ParserOptions>()?;
    m.add_class::<RenderOptions>()?;
    m.add_class::<Output>()?;
    m.add_class::<Warning>()?;
    m.add_function(wrap_pyfunction!(to_html, m)?)?;
    m.add_function(wrap_pyfunction!(noop_loader, m)?)?;
    m.add_function(wrap_pyfunction!(local_loader, m)?)?;
    m.add_function(wrap_pyfunction!(http_loader, m)?)?;
    m.add_function(wrap_pyfunction!(memory_loader, m)?)?;
    m.gil_used(false)?;
    Ok(())
}
