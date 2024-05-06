use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};

use indexmap::{IndexMap, IndexSet};
use rustc_hash::FxHasher;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

type HashImpl = std::hash::BuildHasherDefault<FxHasher>;

pub type MapImpl<K, V> = IndexMap<K, V, HashImpl>;

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(transparent))]
pub struct Map<K, V>(MapImpl<K, V>)
where
    K: Hash + Eq;

impl<K, V> Map<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        Map(MapImpl::with_hasher(HashImpl::default()))
    }

    #[inline]
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        Q: Hash + indexmap::Equivalent<K> + ?Sized,
    {
        self.shift_remove(key)
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

pub type SetImpl<V> = IndexSet<V, HashImpl>;

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(transparent))]
pub struct Set<V>(SetImpl<V>)
where
    V: Hash + Eq;

impl<V> Set<V>
where
    V: Hash + Eq,
{
    pub fn new() -> Self {
        Set(SetImpl::with_hasher(HashImpl::default()))
    }
}

impl<V> Deref for Set<V>
where
    V: Hash + Eq,
{
    type Target = SetImpl<V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<V> DerefMut for Set<V>
where
    V: Hash + Eq,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<V> FromIterator<V> for Set<V>
where
    V: Hash + Eq,
{
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        Set(SetImpl::from_iter(iter))
    }
}
