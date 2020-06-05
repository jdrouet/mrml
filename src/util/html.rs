use super::prelude::sort_by_key;
use std::collections::{HashMap, HashSet};
use std::string::ToString;

pub struct Tag {
    name: String,
    attributes: HashMap<String, String>,
    classes: HashSet<String>,
    styles: HashMap<String, String>,
}

impl Tag {
    pub fn set_class<V: ToString>(mut self, value: V) -> Self {
        self.classes.insert(value.to_string());
        self
    }

    pub fn maybe_set_class<V: ToString>(self, value: Option<V>) -> Self {
        if let Some(content) = value {
            self.set_class(content)
        } else {
            self
        }
    }

    fn classes(&self) -> Option<String> {
        if self.classes.is_empty() {
            None
        } else {
            let mut classes: Vec<String> = self.classes.iter().cloned().collect();
            classes.sort_by(|a, b| a.cmp(&b));
            let classes = classes.join(" ");
            Some(classes)
        }
    }
}

impl Tag {
    pub fn insert_style(self, values: &HashMap<String, String>) -> Self {
        values.iter().fold(self, |res, (key, value)| res.set_style(key, value))
    }

    pub fn set_style<K: ToString, V: ToString>(mut self, key: K, value: V) -> Self {
        self.styles.insert(key.to_string(), value.to_string());
        self
    }

    pub fn maybe_set_style<K: ToString, V: ToString>(self, key: K, value: Option<V>) -> Self {
        if let Some(content) = value {
            self.set_style(key, content)
        } else {
            self
        }
    }

    fn styles(&self) -> Option<String> {
        if self.styles.is_empty() {
            None
        } else {
            let mut style: Vec<(&String, &String)> = self.styles.iter().collect();
            style.sort_by(sort_by_key);
            let style: Vec<String> = style.iter().map(|(k, v)| format!("{}:{};", k, v)).collect();
            Some(style.join(""))
        }
    }
}

impl Tag {
    pub fn set_attribute<K: ToString, V: ToString>(mut self, key: K, value: V) -> Self {
        self.attributes.insert(key.to_string(), value.to_string());
        self
    }

    pub fn maybe_set_attribute<K: ToString, V: ToString>(self, key: K, value: Option<V>) -> Self {
        if let Some(content) = value {
            self.set_attribute(key, content)
        } else {
            self
        }
    }

    fn attributes(&self) -> Option<String> {
        let mut attrs = self.attributes.clone();
        if let Some(classes) = self.classes() {
            attrs.insert("class".to_string(), classes);
        }
        if let Some(style) = self.styles() {
            attrs.insert("style".to_string(), style);
        }
        let mut attrs: Vec<(&String, &String)> = attrs.iter().collect();
        attrs.sort_by(sort_by_key);
        let attrs: Vec<String> = attrs
            .iter()
            .map(|(k, v)| format!("{}=\"{}\"", k, v))
            .collect();
        if attrs.is_empty() {
            None
        } else {
            Some(attrs.join(" "))
        }
    }
}

impl Tag {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            attributes: HashMap::new(),
            classes: HashSet::new(),
            styles: HashMap::new(),
        }
    }

    pub fn open(&self) -> String {
        if let Some(attrs) = self.attributes() {
            format!("<{} {}>", self.name, attrs)
        } else {
            format!("<{}>", self.name)
        }
    }

    pub fn closed(&self) -> String {
        if let Some(attrs) = self.attributes() {
            format!("<{} {} />", self.name, attrs)
        } else {
            format!("<{} />", self.name)
        }
    }

    pub fn close(&self) -> String {
        format!("</{}>", self.name)
    }

    pub fn render<C: ToString>(&self, content: C) -> String {
        format!("{}{}{}", self.open(), content.to_string(), self.close())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_attribute() {
        assert_eq!(
            Tag::new("a")
                .set_attribute("href", "http://localhost")
                .open(),
            "<a href=\"http://localhost\">"
        );
    }

    #[test]
    fn maybe_set_attribute() {
        assert_eq!(
            Tag::new("a")
                .maybe_set_attribute("href", Some("http://localhost"))
                .maybe_set_attribute::<_, i32>("toto", None)
                .open(),
            "<a href=\"http://localhost\">"
        );
    }

    #[test]
    fn set_class() {
        assert_eq!(
            Tag::new("a").set_class("foo").set_class("bar").open(),
            "<a class=\"bar foo\">"
        );
    }

    #[test]
    fn maybe_set_class() {
        assert_eq!(
            Tag::new("a")
                .maybe_set_class(Some("foo"))
                .maybe_set_class::<i32>(None)
                .open(),
            "<a class=\"foo\">"
        );
    }

    #[test]
    fn set_style() {
        assert_eq!(
            Tag::new("a")
                .set_style("foo", 32)
                .set_style("bar", "toto")
                .open(),
            "<a style=\"bar:toto;foo:32;\">"
        );
    }

    #[test]
    fn maybe_set_style() {
        assert_eq!(
            Tag::new("a")
                .maybe_set_style("foo", Some(32))
                .maybe_set_style::<_, i32>("bar", None)
                .open(),
            "<a style=\"foo:32;\">"
        );
    }

    #[test]
    fn closed() {
        assert_eq!(
            Tag::new("a")
                .set_attribute("toto", "tata")
                .maybe_set_style("foo", Some(32))
                .maybe_set_style::<_, i32>("bar", None)
                .closed(),
            "<a style=\"foo:32;\" toto=\"tata\" />"
        );
    }

    #[test]
    fn close() {
        assert_eq!(Tag::new("a").close(), "</a>");
    }
}
