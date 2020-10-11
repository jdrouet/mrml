pub mod attributes;
pub mod condition;
pub mod context;
pub mod fonts;
pub mod header;
pub mod id;
pub mod size;
pub mod spacing;
pub mod style;
pub mod tag;

use std::cmp::Ordering;

pub fn sort_by_key<'r, 's>(a: &'r (&String, &String), b: &'s (&String, &String)) -> Ordering {
    a.0.partial_cmp(&b.0).unwrap()
}
