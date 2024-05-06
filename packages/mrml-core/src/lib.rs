//! This project is a reimplementation of the nice [MJML](https://mjml.io/) markup language in Rust.
//!
//! [![.github/workflows/main.yml](https://github.com/jdrouet/mrml/actions/workflows/mrml-core-main.yml/badge.svg)](https://github.com/jdrouet/mrml/actions/workflows/mrml-core-main.yml)
//! [![codecov](https://codecov.io/gh/jdrouet/mrml/branch/main/graph/badge.svg?token=SIOPR0YWZA)](https://codecov.io/gh/jdrouet/mrml)
//! [![Maintainability](https://api.codeclimate.com/v1/badges/7ed23ef670d076ab69a4/maintainability)](https://codeclimate.com/github/jdrouet/mrml/maintainability)
//!
//! # How to use?
//!
//! To use it you can simply update your `Cargo.toml` by adding
//! ```toml
//! [dependencies]
//! mrml = { version = "3" }
//! serde = { version = "1", features = ["derive"] }
//! ```
//!
//! And you can then just create a `main.rs` with the following code
//! ```rust
//! # #[cfg(feature = "parse")]
//! # {
//! let root = mrml::parse("<mjml><mj-body></mj-body></mjml>").expect("parse template");
//! let opts = mrml::prelude::render::Options::default();
//! match root.render(&opts) {
//!     Ok(content) => println!("{}", content),
//!     Err(_) => println!("couldn't render mjml template"),
//! };
//! # }
//! ```
//!
//! ## Using `mj-include`
//!
//! You can also use the `mj-include` component by specifying a
//! [loader](crate::prelude::parser).
//!
//! ```rust
//! # #[cfg(feature = "parse")]
//! # {
//! use mrml::prelude::parser::ParserOptions;
//! use mrml::prelude::parser::memory_loader::MemoryIncludeLoader;
//!
//! let loader = MemoryIncludeLoader::from(vec![("partial.mjml", "<mj-button>Hello</mj-button>")]);
//! let options = ParserOptions {
//!     include_loader: Box::new(loader),
//! };
//! match mrml::parse_with_options("<mjml><mj-head /><mj-body><mj-include path=\"partial.mjml\" /></mj-body></mjml>", &options) {
//!     Ok(_) => println!("Success!"),
//!     Err(err) => eprintln!("Something went wrong: {err:?}"),
//! }
//! # }
//! ```
//!
//! ## Using `mj-include` with an async loader
//!
//! If you want to use the async version to fetch the includes, you've to enable the `async` feature and the required loaders (`http-loader-async-reqwest` in this example).
//!
//! ```rust
//! # #[cfg(all(feature = "parse", feature = "render", feature = "async"))]
//! # tokio_test::block_on(async {
//! use mrml::prelude::parser::http_loader::{AsyncReqwestFetcher, HttpIncludeLoader};
//! use mrml::prelude::parser::memory_loader::MemoryIncludeLoader;
//! use mrml::prelude::parser::local_loader::LocalIncludeLoader;
//! use mrml::prelude::parser::multi_loader::MultiIncludeLoader;
//! use mrml::prelude::parser::noop_loader::NoopIncludeLoader;
//! use mrml::prelude::parser::loader::AsyncIncludeLoader;
//! use mrml::prelude::parser::AsyncParserOptions;
//! use mrml::prelude::render::RenderOptions;
//! use std::path::PathBuf;
//! use std::sync::Arc;
//!
//! let resolver = MultiIncludeLoader::<Box<dyn AsyncIncludeLoader + Send + Sync + 'static>>::new()
//!     .with_starts_with("memory://", Box::new(MemoryIncludeLoader::from(vec![("basic.mjml", "<mj-button>Hello</mj-button>")])))
//!     .with_starts_with("file://", Box::new(LocalIncludeLoader::new(PathBuf::default().join("resources").join("compare").join("success"))))
//!     .with_starts_with("https://", Box::new(HttpIncludeLoader::<AsyncReqwestFetcher>::allow_all()))
//!     .with_any(Box::<NoopIncludeLoader>::default());
//! let parser_options = AsyncParserOptions {
//!     include_loader: Box::new(resolver),
//! };
//! let render_options = RenderOptions::default();
//! let json = r#"<mjml>
//! <mj-body>
//! <mj-include path="file://basic.mjml" />
//! <mj-include path="memory://basic.mjml" />
//! </mj-body>
//! </mjml>"#;
//! match mrml::async_parse_with_options(json, Arc::new(parser_options)).await {
//!     Ok(mjml) => match mjml.render(&render_options) {
//!         Ok(html) => println!("{html}"),
//!         Err(err) => eprintln!("Couldn't render template: {err:?}"),
//!     },
//!     Err(err) => eprintln!("Couldn't parse template: {err:?}"),
//! }
//! # })
//! ```
//!
//! ## Using `mrml` in Python
//!
//! This crate can also be used in Python. The crate is available with pypi and
//! you can find some documentation [here](https://pypi.org/project/mrml/).
//!
//! ```python
//! import mrml
//!
//! # without options
//! result = mrml.to_html("<mjml></mjml>")
//! assert result.startswith("<!doctype html>")
//!
//! # with options
//! parser_options = mrml.ParserOptions(include_loader = mrml.memory_loader({
//!     'hello-world.mjml': '<mj-text>Hello World!</mj-text>',
//! }))
//! result = mrml.to_html("<mjml><mj-body><mj-include path=\"hello-world.mjml\" /></mj-body></mjml>", parser_options = parser_options)
//! assert result.startswith("<!doctype html>")
//! ```
//!
//! # Why?
//!
//! A Node.js server rendering an MJML template takes around **20 MB** of RAM at
//! startup and **130 MB** under stress test. In Rust, less than **1.7 MB** at
//! startup and a bit less that **3 MB** under stress test. The Rust version can
//! also handle twice as many requests per second. You can perform the
//! benchmarks by running `bash script/run-bench.sh`.
//!
//! Also, the JavaScript implementation cannot be run in the browser; the Rust
//! one (and WebAssembly one) can be.

pub mod comment;
pub mod mj_accordion;
pub mod mj_accordion_element;
pub mod mj_accordion_text;
pub mod mj_accordion_title;
pub mod mj_attributes;
pub mod mj_attributes_all;
pub mod mj_attributes_class;
pub mod mj_attributes_element;
pub mod mj_body;
pub mod mj_breakpoint;
pub mod mj_button;
pub mod mj_carousel;
pub mod mj_carousel_image;
pub mod mj_column;
pub mod mj_divider;
pub mod mj_font;
pub mod mj_group;
pub mod mj_head;
pub mod mj_hero;
pub mod mj_image;
pub mod mj_include;
pub mod mj_navbar;
pub mod mj_navbar_link;
pub mod mj_preview;
pub mod mj_raw;
pub mod mj_section;
pub mod mj_social;
pub mod mj_social_element;
pub mod mj_spacer;
pub mod mj_style;
pub mod mj_table;
pub mod mj_text;
pub mod mj_title;
pub mod mj_wrapper;
pub mod mjml;
pub mod node;
pub mod prelude;
pub mod text;

// Only used to ignore the comments at the root level
mod root;

mod helper;

#[cfg(feature = "parse")]
/// Function to parse a raw mjml template with some parsing
/// [options](crate::prelude::parser::ParserOptions). This function is just an
/// alias to [the `Mjml::parse_with_options` function](crate::mjml::Mjml).
///
/// You can specify the kind of loader mrml needs to use for loading the content
/// of [`mj-include`](crate::mj_include) elements.
///
/// You can take a look at the available loaders [here](crate::prelude::parser).
///
/// ```rust
/// use mrml::prelude::parser::ParserOptions;
/// use mrml::prelude::parser::memory_loader::MemoryIncludeLoader;
///
/// let options = ParserOptions {
///     include_loader: Box::new(MemoryIncludeLoader::default()),
/// };
/// match mrml::parse_with_options("<mjml><mj-head /><mj-body /></mjml>", &options) {
///     Ok(_) => println!("Success!"),
///     Err(err) => eprintln!("Something went wrong: {err:?}"),
/// }
/// ```
pub fn parse_with_options<T: AsRef<str>>(
    input: T,
    opts: &crate::prelude::parser::ParserOptions,
) -> Result<mjml::Mjml, prelude::parser::Error> {
    let root = crate::root::Root::parse_with_options(input, opts)?;
    root.into_mjml().ok_or(prelude::parser::Error::NoRootNode)
}

#[cfg(all(feature = "parse", feature = "async"))]
/// Function to parse asynchronously a raw mjml template with some parsing
/// [options](crate::prelude::parser::AsyncParserOptions). This function is just an
/// alias to [the `Mjml::async_parse_with_options` function](crate::mjml::Mjml).
///
/// You can specify the kind of loader mrml needs to use for loading the content
/// of [`mj-include`](crate::mj_include) elements.
///
/// You can take a look at the available loaders [here](crate::prelude::parse).
///
/// ```rust
/// # tokio_test::block_on(async {
/// use mrml::prelude::parser::AsyncParserOptions;
/// use mrml::prelude::parser::memory_loader::MemoryIncludeLoader;
///
/// let options = std::sync::Arc::new(AsyncParserOptions {
///     include_loader: Box::new(MemoryIncludeLoader::default()),
/// });
/// match mrml::async_parse_with_options("<mjml><mj-head /><mj-body /></mjml>", options).await {
///     Ok(_) => println!("Success!"),
///     Err(err) => eprintln!("Something went wrong: {err:?}"),
/// }
/// # })
/// ```
pub async fn async_parse_with_options<T: AsRef<str>>(
    input: T,
    opts: std::sync::Arc<crate::prelude::parser::AsyncParserOptions>,
) -> Result<mjml::Mjml, prelude::parser::Error> {
    let root = crate::root::Root::async_parse_with_options(input, opts).await?;
    root.into_mjml().ok_or(prelude::parser::Error::NoRootNode)
}

#[cfg(feature = "parse")]
/// Function to parse a raw mjml template using the default parsing
/// [options](crate::prelude::parser::ParserOptions).
///
/// ```rust
/// match mrml::parse("<mjml><mj-head /><mj-body /></mjml>") {
///     Ok(_) => println!("Success!"),
///     Err(err) => eprintln!("Something went wrong: {err:?}"),
/// }
/// ```
pub fn parse<T: AsRef<str>>(input: T) -> Result<mjml::Mjml, prelude::parser::Error> {
    let opts = crate::prelude::parser::ParserOptions::default();
    parse_with_options(input, &opts)
}

#[cfg(all(feature = "parse", feature = "async"))]
/// Function to parse a raw mjml template using the default parsing
/// [options](crate::prelude::parser::ParserOptions).
///
/// ```rust
/// # tokio_test::block_on(async {
/// match mrml::async_parse("<mjml><mj-head /><mj-body /></mjml>").await {
///     Ok(_) => println!("Success!"),
///     Err(err) => eprintln!("Something went wrong: {err:?}"),
/// }
/// # })
/// ```
pub async fn async_parse<T: AsRef<str>>(input: T) -> Result<mjml::Mjml, prelude::parser::Error> {
    let opts = std::sync::Arc::new(crate::prelude::parser::AsyncParserOptions::default());
    async_parse_with_options(input, opts).await
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    #[test]
    fn parse_simple() {
        let _ = crate::parse("<mjml><mj-head /><mj-body /></mjml>");
    }

    #[test]
    fn parse_with_options() {
        let _ =
            crate::parse_with_options("<mjml><mj-head /><mj-body /></mjml>", &Default::default());
    }
}
