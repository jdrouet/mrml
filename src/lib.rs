#[cfg(test)]
#[macro_use(assert_diff)]
extern crate difference;

#[macro_use]
extern crate lazy_static;

use std::default::Default;

pub mod elements;
mod error;
pub mod parser;
pub mod util;

pub use error::Error;
use util::fonts::FontRegistry;
use util::size::Size;

/// global options for renderer
#[derive(Clone, Debug)]
pub struct Options {
    /// size between mobile and desktop
    pub breakpoint: Size,
    /// list of registered fonts
    pub fonts: FontRegistry,
    /// rather the comments should be kept
    pub keep_comments: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            breakpoint: Size::Pixel(480.0),
            fonts: FontRegistry::new(),
            keep_comments: true,
        }
    }
}

pub fn parse(input: &str, options: Options) -> Result<elements::MJMLElement, Error> {
    let root = parser::parse(input)?;
    let element = elements::parse(&root, options)?;
    Ok(element)
}

/// generate the title from mjml
///
/// ```rust
/// use mrml::{to_title, Options};
/// let result = to_title(r#"
///     <mjml>
///         <mj-head>
///             <mj-title>Something</mj-title>
///         </mj-head>
///     </mjml>
/// "#, Options::default());
/// assert!(result.is_ok());
/// assert_eq!(result.unwrap(), "Something");
/// ```
///
/// ```rust
/// use mrml::{to_title, Options};
/// let result = to_title("<mjml", Options::default());
/// assert!(result.is_err());
/// ```
pub fn to_title(input: &str, options: Options) -> Result<String, Error> {
    let element = parse(input, options)?;
    Ok(element.get_title())
}

/// generate the preview from mjml
///
/// ```rust
/// use mrml::{to_preview, Options};
/// let result = to_preview("<mjml></mjml>", Options::default());
/// assert!(result.is_ok());
/// ```
///
/// ```rust
/// use mrml::{to_preview, Options};
/// let result = to_preview("<mjml", Options::default());
/// assert!(result.is_err());
/// ```
pub fn to_preview(input: &str, options: Options) -> Result<String, Error> {
    let element = parse(input, options)?;
    Ok(element.get_preview())
}

/// generate the html from mjml
///
/// ```rust
/// use mrml::{to_html, Options};
/// let result = to_html("<mjml></mjml>", Options::default());
/// assert!(result.is_ok());
/// ```
///
/// ```rust
/// use mrml::{to_html, Options};
/// let result = to_html("<mjml", Options::default());
/// assert!(result.is_err());
/// ```
pub fn to_html(input: &str, options: Options) -> Result<String, Error> {
    let element = parse(input, options)?;
    Ok(element.get_html()?)
}

#[derive(Clone, Debug)]
pub struct Email {
    pub subject: String,
    pub text: String,
    pub html: String,
}

/// generate an email from mjml
///
/// ```rust
/// use mrml::{to_email, Options};
/// let result = to_email(r#"
///     <mjml>
///         <mj-head>
///             <mj-title>Testing</mj-title>
///             <mj-preview>Preview</mj-preview>
///         </mj-head>
///     </mjml>
/// "#, Options::default());
/// assert!(result.is_ok());
/// let result = result.unwrap();
/// assert_eq!(result.subject, "Testing");
/// assert_eq!(result.text, "Preview");
/// ```
///
/// ```rust
/// use mrml::{to_email, Options};
/// let result = to_email("<mjml", Options::default());
/// assert!(result.is_err());
/// ```
pub fn to_email(input: &str, options: Options) -> Result<Email, Error> {
    let element = parse(input, options)?;
    let subject = element.get_title();
    let text = element.get_preview();
    let html = element.get_html()?;
    Ok(Email {
        subject,
        text,
        html,
    })
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::env;

    #[test]
    fn invalid_mjml() {
        let res = to_html("<mjml", Options::default());
        assert!(res.is_err());
    }

    fn diff(first: &str, second: &str) {
        if env::var("BASIC_DIFF").is_ok() {
            assert_eq!(first, second);
        } else {
            assert_diff!(first, second, "", 0);
        }
    }

    fn clean_str(input: String) -> String {
        input
            .replace("class=\"\"", "")
            .replace("style=\"\"", "")
            .replace("\n", "")
            .replace("\t", "")
            .replace(" ", "")
            .replace("<![endif]--><!--[ifmso|IE]>", "")
    }

    pub fn compare_title(source: &str, expected: &str) {
        let result = to_title(source, Options::default()).unwrap();
        assert_diff!(result.as_str(), expected, "", 0);
    }

    pub fn compare_preview(source: &str, expected: &str) {
        let result = to_preview(source, Options::default()).unwrap();
        assert_diff!(result.as_str(), expected, "", 0);
    }

    pub fn compare_render(source: &str, expected: &str) {
        let result = to_html(source, Options::default()).unwrap();
        let result = clean_str(result);
        let expected = clean_str(expected.into());
        diff(result.as_str(), expected.as_str());
    }

    pub fn compare_render_with_options(source: &str, expected: &str, opts: Options) {
        let result = to_html(source, opts).unwrap();
        let result = clean_str(result);
        let expected = clean_str(expected.into());
        diff(result.as_str(), expected.as_str());
    }
}
