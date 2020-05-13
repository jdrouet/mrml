use std::collections::HashMap;
use std::fmt::Display;

fn sort_by_key<'r, 's>(a: &'r (String, String), b: &'s (String, String)) -> std::cmp::Ordering {
    a.0.partial_cmp(&b.0).unwrap()
}

#[derive(Clone, Debug)]
pub struct Properties {
    inner: HashMap<String, String>,
}

impl Properties {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn from(other: HashMap<String, String>) -> Self {
        let mut new = Self::new();
        for (key, value) in other.iter() {
            new.inner.insert(key.to_string(), value.to_string());
        }
        new
    }

    pub fn inner(&self) -> HashMap<String, String> {
        self.inner.clone()
    }

    pub fn merge(&mut self, other: &Self) {
        for (key, value) in other.inner.iter() {
            self.inner.insert(key.to_string(), value.to_string());
        }
    }

    pub fn is_empty(&self) -> bool {
        self.inner().is_empty()
    }

    pub fn get<K: Display>(&self, key: K) -> Option<String> {
        let key = key.to_string();
        self.inner().get(&key).and_then(|v| Some(v.to_string()))
    }

    pub fn set<K: Display, V: Display>(&mut self, key: K, value: V) {
        self.inner.insert(key.to_string(), value.to_string());
    }

    pub fn maybe_set<K: Display, V: Display>(&mut self, key: K, value: Option<V>) {
        if let Some(value) = value {
            self.set(key, value);
        }
    }

    fn get_entries(&self) -> Vec<(String, String)> {
        self.inner
            .iter()
            .map(|v| (v.0.clone(), v.1.clone()))
            .collect()
    }

    fn get_sorted_entries(&self) -> Vec<(String, String)> {
        let mut entries = self.get_entries();
        entries.sort_by(sort_by_key);
        entries
    }

    pub fn as_style(&self) -> String {
        let entries = self.get_sorted_entries();
        entries
            .iter()
            .map(|v| format!("{}:{};", v.0, v.1))
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn as_attributes(&self) -> String {
        let entries = self.get_sorted_entries();
        entries
            .iter()
            .map(|v| format!("{}=\"{}\"", v.0, v.1))
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_style() {
        let mut props = Properties::new();
        props.set("border-right", "3px");
        props.set("border", "1px");
        props.set("border-left", "2px");
        assert_eq!(
            props.as_style(),
            "border:1px;border-left:2px;border-right:3px;"
        );
    }
}
