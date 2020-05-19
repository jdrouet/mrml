use super::prelude::{sort_by_key, PropertyMap};
use std::collections::HashMap;
use std::string::ToString;

pub struct Style {
    inner: HashMap<String, String>,
}

impl PropertyMap for Style {
    fn inner(&self) -> &HashMap<String, String> {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.inner
    }
}

impl ToString for Style {
    fn to_string(&self) -> String {
        let mut entries = self.get_entries();
        entries.sort_by(sort_by_key);
        entries
            .iter()
            .map(|v| format!("{}:{};", v.0, v.1))
            .collect::<Vec<String>>()
            .join("")
    }
}

impl Style {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
}
