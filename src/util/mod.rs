mod attributes;
pub mod condition;
mod context;
pub mod fonts;
mod header;
mod size;
mod spacing;
mod style;
mod tag;

pub use attributes::{suffix_css_classes, suffix_unit, Attributes};
pub use context::Context;
pub use header::Header;
pub use size::Size;
pub use spacing::Spacing;
pub use style::Style;
pub use tag::Tag;

use std::cmp::Ordering;

pub fn sort_by_key<'r, 's>(a: &'r (&String, &String), b: &'s (&String, &String)) -> Ordering {
    a.0.partial_cmp(&b.0).unwrap()
}
