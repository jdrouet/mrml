mod attributes;
pub mod condition;
mod context;
pub mod fonts;
mod header;
mod html;
mod size;
mod spacing;

pub use attributes::{suffix_css_classes, suffix_unit, Attributes};
pub use context::Context;
pub use header::Header;
pub use html::Tag;
pub use html::Style;
pub use size::Size;
pub use spacing::Spacing;

use std::cmp::Ordering;

pub fn sort_by_key<'r, 's>(a: &'r (&String, &String), b: &'s (&String, &String)) -> Ordering {
    a.0.partial_cmp(&b.0).unwrap()
}
