use std::{
    hash::Hash,
    iter::FromIterator,
    ops::{Deref, DerefMut},
};

use indexmap::IndexMap;
use rustc_hash::FxHasher;
use serde::{Deserialize, Serialize};

type HashImpl = std::hash::BuildHasherDefault<FxHasher>;
pub type MapImpl<K, V> = IndexMap<K, V, HashImpl>;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Map<K, V>(MapImpl<K, V>)
where
    K: Hash + Eq;

impl<K, V> Map<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        // Map(MapImpl::new())
        Map(MapImpl::with_hasher(HashImpl::default()))
    }
}

impl<K, V> Deref for Map<K, V>
where
    K: Hash + Eq,
{
    type Target = MapImpl<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> DerefMut for Map<K, V>
where
    K: Hash + Eq,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K, V> FromIterator<(K, V)> for Map<K, V>
where
    K: Hash + Eq,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Map(MapImpl::from_iter(iter))
    }
}
