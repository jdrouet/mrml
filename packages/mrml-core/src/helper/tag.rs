use std::collections::{HashSet};

use crate::prelude::hash::Map;

pub struct Tag {
    name: String,
    attributes: Map<String, String>,
    classes: HashSet<String>,
    // in order to keep the style in the same order the've been added
    styles: Vec<(String, String)>,
}

impl Tag {
    pub fn table() -> Self {
        Self::new("table")
    }
    pub fn table_borderless() -> Self {
        Self::table()
            .add_attribute("border", "0")
            .add_attribute("cellpadding", "0")
            .add_attribute("cellspacing", "0")
    }
    pub fn table_presentation() -> Self {
        Self::table_borderless().add_attribute("role", "presentation")
    }
    pub fn tbody() -> Self {
        Self::new("tbody")
    }
    pub fn tr() -> Self {
        Self::new("tr")
    }
    pub fn td() -> Self {
        Self::new("td")
    }
    pub fn div() -> Self {
        Self::new("div")
    }

    pub fn new<T: ToString>(name: T) -> Self {
        Self {
            name: name.to_string(),
            attributes: Map::new(),
            classes: HashSet::new(),
            styles: Vec::new(),
        }
    }

    pub fn add_class<T: ToString>(mut self, value: T) -> Self {
        self.classes.insert(value.to_string());
        self
    }

    pub fn add_suffixed_class<T: AsRef<str>>(self, value: T, suffix: &str) -> Self {
        self.add_class(format!("{}-{}", value.as_ref(), suffix))
    }

    pub fn maybe_add_suffixed_class<T: AsRef<str>>(self, value: Option<T>, suffix: &str) -> Self {
        if let Some(value) = value {
            self.add_suffixed_class(value, suffix)
        } else {
            self
        }
    }

    pub fn maybe_add_class<T: ToString>(self, value: Option<T>) -> Self {
        if let Some(value) = value {
            self.add_class(value)
        } else {
            self
        }
    }

    pub fn add_attribute<V: ToString>(mut self, name: &str, value: V) -> Self {
        self.attributes.insert(name.to_string(), value.to_string());
        self
    }

    pub fn maybe_add_attribute<T: ToString>(self, name: &str, value: Option<T>) -> Self {
        if let Some(value) = value {
            self.add_attribute(name, value)
        } else {
            self
        }
    }

    pub fn add_style<T: ToString>(mut self, name: &str, value: T) -> Self {
        self.styles.push((name.to_string(), value.to_string()));
        self
    }

    pub fn maybe_add_style<T: ToString>(self, name: &str, value: Option<T>) -> Self {
        if let Some(value) = value {
            self.add_style(name, value)
        } else {
            self
        }
    }

    fn opening(&self) -> String {
        let mut res = String::from("<");
        res.push_str(&self.name);
        for (key, value) in self.attributes.iter() {
            res.push(' ');
            res.push_str(key);
            res.push_str("=\"");
            res.push_str(value);
            res.push('"');
        }
        if !self.classes.is_empty() {
            res.push_str(" class=\"");
            for (index, classname) in self.classes.iter().enumerate() {
                if index > 0 {
                    res.push(' ');
                }
                res.push_str(classname);
            }
            res.push('"');
        }
        if !self.styles.is_empty() {
            res.push_str(" style=\"");
            for (key, value) in self.styles.iter() {
                res.push_str(key);
                res.push(':');
                res.push_str(value);
                res.push(';');
            }
            res.push('"');
        }
        res
    }

    pub fn open(&self) -> String {
        self.opening() + ">"
    }

    pub fn close(&self) -> String {
        format!("</{}>", self.name)
    }

    pub fn closed(&self) -> String {
        self.opening() + " />"
    }

    pub fn render<T: AsRef<str>>(&self, input: T) -> String {
        self.open() + input.as_ref() + &self.close()
    }
}
