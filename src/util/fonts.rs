use super::tag::Tag;
use std::collections::HashMap;
use std::string::ToString;

#[derive(Clone, Debug)]
pub struct FontRegistry {
    inner: HashMap<String, String>,
}

pub fn url_to_link(url: &str) -> String {
    Tag::new("link")
        .set_attribute("href", url)
        .set_attribute("rel", "stylesheet")
        .set_attribute("type", "text/css")
        .open()
}

pub fn url_to_import(url: &str) -> String {
    format!("@import url({});", url)
}

impl FontRegistry {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn default() -> Self {
        let mut item = Self::new();
        item.add_google_fonts("Open Sans");
        item.add_google_fonts("Droid Sans");
        item.add_google_fonts("Lato");
        item.add_google_fonts("Roboto");
        item.add_google_fonts("Ubuntu");
        item
    }

    pub fn add(&mut self, name: &str, url: &str) {
        self.inner.insert(name.to_string(), url.to_string());
    }

    fn add_google_fonts(&mut self, name: &str) {
        let url = format!(
            "https://fonts.googleapis.com/css?family={}:300,400,500,700",
            name.replace(" ", "+")
        );
        self.inner.insert(name.to_string(), url);
    }

    pub fn get(&self, name: &String) -> Option<&String> {
        self.inner.get(name)
    }
}
