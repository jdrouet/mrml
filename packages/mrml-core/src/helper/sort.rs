use std::cmp::Ordering;

pub fn sort_by_key<V>(a: &(&String, V), b: &(&String, V)) -> Ordering {
    a.0.partial_cmp(b.0).unwrap()
}
