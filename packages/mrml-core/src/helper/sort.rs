use std::cmp::Ordering;

pub fn sort_by_key<'r, 's, V>(a: &'r (&String, V), b: &'s (&String, V)) -> Ordering {
    a.0.partial_cmp(&b.0).unwrap()
}
