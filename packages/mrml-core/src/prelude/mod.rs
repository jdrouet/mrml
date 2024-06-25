#[cfg(feature = "parse")]
pub mod parser;
#[cfg(feature = "print")]
pub mod print;
#[cfg(feature = "render")]
pub mod render;

pub mod hash;

// see https://developer.mozilla.org/en-US/docs/Glossary/Void_element
#[cfg(any(feature = "parse", feature = "print", feature = "render"))]
pub(crate) fn is_void_element(tag: &str) -> bool {
    matches!(
        tag,
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "param"
            | "source"
            | "track"
            | "wbr"
    )
}
