use super::sort_by_key;
use std::collections::HashMap;
use std::string::ToString;

pub struct Attributes(HashMap<String, String>);

impl Attributes {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn inner(&self) -> &HashMap<String, String> {
        &self.0
    }

    pub fn get<K: ToString>(&self, key: K) -> Option<&String> {
        self.0.get(&key.to_string())
    }

    pub fn set<K: ToString, V: ToString>(&mut self, key: K, value: V) {
        self.0.insert(key.to_string(), value.to_string());
    }

    pub fn add<K: ToString, V: ToString>(mut self, key: K, value: V) -> Self {
        self.set(key, value);
        self
    }

    pub fn maybe_set<K: ToString, V: ToString>(&mut self, key: K, value: Option<V>) {
        if let Some(content) = value {
            self.set(key, content);
        }
    }

    pub fn maybe_add<K: ToString, V: ToString>(mut self, key: K, value: Option<V>) -> Self {
        self.maybe_set(key, value);
        self
    }

    pub fn entries(&self) -> Vec<(&String, &String)> {
        self.0.iter().collect()
    }
}

impl From<&Attributes> for Attributes {
    fn from(value: &Attributes) -> Self {
        Self(value.0.clone())
    }
}

impl ToString for Attributes {
    fn to_string(&self) -> String {
        let mut entries = self.entries();
        entries.sort_by(sort_by_key);
        entries
            .iter()
            .map(|v| format!("{}=\"{}\"", v.0, v.1))
            .collect::<Vec<String>>()
            .join(" ")
    }
}

pub fn suffix_unit(input: Option<String>, suffix: &str) -> Option<String> {
    input.and_then(|v| Some(format!("{}{}", v, suffix)))
}

pub fn suffix_css_classes(input: Option<String>, suffix: &str) -> Option<String> {
    if let Some(value) = input {
        let value: Vec<String> = value
            .split(" ")
            .filter(|v| v.len() > 0)
            .map(|v| format!("{}-{}", v, suffix))
            .collect();
        if value.is_empty() {
            None
        } else {
            Some(value.join(" "))
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn suffix_css_classes_none() {
        assert_eq!(suffix_css_classes(None, "whatever"), None);
    }

    #[test]
    fn suffix_css_classes_some_empty() {
        assert_eq!(suffix_css_classes(Some("".into()), "whatever"), None);
    }

    #[test]
    fn suffix_css_classes_with_values() {
        assert_eq!(
            suffix_css_classes(Some("toto tutu".into()), "whatever"),
            Some("toto-whatever tutu-whatever".into())
        );
    }
}
