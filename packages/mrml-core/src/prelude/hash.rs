use indexmap::{IndexMap, IndexSet};
use rustc_hash::FxBuildHasher;

pub type Map<K, V> = IndexMap<K, V, FxBuildHasher>;
pub type Set<V> = IndexSet<V, FxBuildHasher>;
