use std::cmp::Ordering;
use std::collections::HashMap;
use std::string::ToString;

pub fn sort_by_key<'r, 's>(a: &'r (&String, &String), b: &'s (&String, &String)) -> Ordering {
    a.0.partial_cmp(&b.0).unwrap()
}

pub trait Properties {
    fn inner(&self) -> &HashMap<String, String>;
    fn inner_mut(&mut self) -> &mut HashMap<String, String>;

    fn is_empty(&self) -> bool {
        self.inner().is_empty()
    }

    fn merge(&mut self, other: &Self) {
        for (key, value) in other.inner().iter() {
            self.inner_mut().insert(key.to_string(), value.to_string());
        }
    }

    fn get<K: ToString>(&self, key: K) -> Option<&String> {
        self.inner().get(&key.to_string())
    }

    fn set<K: ToString, V: ToString>(&mut self, key: K, value: V) {
        self.inner_mut().insert(key.to_string(), value.to_string());
    }

    fn maybe_set<K: ToString, V: ToString>(&mut self, key: K, value: Option<V>) {
        if let Some(content) = value {
            self.set(key, content);
        }
    }

    fn entries(&self) -> Vec<(&String, &String)> {
        self.inner().iter().collect()
    }
}
