#![doc = include_str!("../readme.md")]

pub mod comment;
pub mod conditional_comment;
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
#[cfg(feature = "parse")]
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
) -> Result<crate::prelude::parser::ParseOutput<mjml::Mjml>, prelude::parser::Error> {
    let root = crate::root::Root::parse_with_options(input, opts)?;
    Ok(crate::prelude::parser::ParseOutput {
        element: root
            .element
            .into_mjml()
            .ok_or(prelude::parser::Error::NoRootNode)?,
        warnings: root.warnings,
    })
}

#[cfg(all(feature = "parse", feature = "async"))]
/// Function to parse asynchronously a raw mjml template with some parsing
/// [options](crate::prelude::parser::AsyncParserOptions). This function is just
/// an alias to [the `Mjml::async_parse_with_options`
/// function](crate::mjml::Mjml).
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
) -> Result<crate::prelude::parser::ParseOutput<mjml::Mjml>, prelude::parser::Error> {
    let root = crate::root::Root::async_parse_with_options(input, opts).await?;
    Ok(crate::prelude::parser::ParseOutput {
        element: root
            .element
            .into_mjml()
            .ok_or(prelude::parser::Error::NoRootNode)?,
        warnings: root.warnings,
    })
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
pub fn parse<T: AsRef<str>>(
    input: T,
) -> Result<prelude::parser::ParseOutput<mjml::Mjml>, prelude::parser::Error> {
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
pub async fn async_parse<T: AsRef<str>>(
    input: T,
) -> Result<crate::prelude::parser::ParseOutput<mjml::Mjml>, prelude::parser::Error> {
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
