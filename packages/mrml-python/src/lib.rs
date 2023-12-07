use std::borrow::Cow;
use std::collections::HashMap;

use mrml::prelude::parser::loader::IncludeLoader;
use mrml::prelude::parser::memory_loader::MemoryIncludeLoader;
use mrml::prelude::parser::noop_loader::NoopIncludeLoader;
use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug, Default)]
pub struct NoopIncludeLoaderOptions;

#[pyclass]
#[derive(Clone, Debug, Default)]
pub struct MemoryIncludeLoaderOptions(HashMap<String, String>);

// #[pyclass]
#[derive(FromPyObject, Clone, Debug)]
pub enum ParserIncludeLoaderOptions {
    Noop(NoopIncludeLoaderOptions),
    Memory(MemoryIncludeLoaderOptions),
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
        }
    }
}

impl IntoPy<PyObject> for ParserIncludeLoaderOptions {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Self::Noop(inner) => inner.into_py(py),
            Self::Memory(inner) => inner.into_py(py),
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

#[pyclass]
#[derive(Clone, Debug, Default)]
pub struct ParserOptions {
    #[pyo3(get, set)]
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

#[pyclass]
#[derive(Clone, Debug, Default)]
pub struct RenderOptions {
    #[pyo3(get, set)]
    pub disable_comments: bool,
    #[pyo3(get, set)]
    pub social_icon_origin: Option<String>,
    #[pyo3(get, set)]
    pub fonts: Option<HashMap<String, String>>,
}

#[pymethods]
impl RenderOptions {
    #[new]
    pub fn new() -> Self {
        Self::default()
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

#[pyfunction]
#[pyo3(name = "to_html", signature = (input, parser_options=None, render_options=None))]
fn to_html(
    input: String,
    parser_options: Option<ParserOptions>,
    render_options: Option<RenderOptions>,
) -> PyResult<String> {
    let parser_options = parser_options.unwrap_or_default().into();
    let parsed = mrml::parse_with_options(input, &parser_options)
        .map_err(|err| PyOSError::new_err(err.to_string()))?;

    let render_options = render_options.unwrap_or_default().into();
    let output = parsed
        .render(&render_options)
        .map_err(|err| PyOSError::new_err(err.to_string()))?;

    Ok(output)
}

#[pymodule]
#[pyo3(name = "mrml")]
fn register(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<NoopIncludeLoaderOptions>()?;
    m.add_class::<MemoryIncludeLoaderOptions>()?;
    m.add_class::<ParserOptions>()?;
    m.add_class::<RenderOptions>()?;
    m.add_function(wrap_pyfunction!(to_html, m)?)?;
    m.add_function(wrap_pyfunction!(noop_loader, m)?)?;
    m.add_function(wrap_pyfunction!(memory_loader, m)?)?;
    Ok(())
}
