use super::prelude::*;
use std::collections::HashMap;
use std::string::ToString;

pub struct Style(HashMap<String, String>);

impl Properties for Style {
    fn inner(&self) -> &HashMap<String, String> {
        &self.0
    }

    fn inner_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.0
    }
}

impl ToString for Style {
    fn to_string(&self) -> String {
        let mut entries = self.entries();
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
        Self(HashMap::new())
    }
}
