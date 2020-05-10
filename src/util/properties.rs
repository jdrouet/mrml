use std::collections::HashMap;
use std::fmt::Display;

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

    fn get_attribute_list(&self) -> Vec<String> {
        let mut result = vec![];
        for (key, value) in self.inner.iter() {
            result.push(format!("{}=\"{}\"", key, value));
        }
        result.sort();
        result
    }

    fn get_style_list(&self) -> Vec<String> {
        let mut result = vec![];
        for (key, value) in self.inner.iter() {
            result.push(format!("{}:{};", key, value));
        }
        result.sort();
        result
    }

    pub fn as_style(&self) -> String {
        self.get_style_list().join("")
    }

    pub fn as_attributes(&self) -> String {
        self.get_attribute_list().join(" ")
    }
}
