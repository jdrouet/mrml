use super::fonts::FontRegistry;
use super::Size;
use crate::Options;
use std::collections::HashMap;
use std::collections::HashSet;
use std::string::ToString;

#[derive(Clone, Debug)]
pub struct Header {
    breakpoint: Size,
    font_registry: FontRegistry,
    font_families: HashSet<String>,
    title: Option<String>,
    media_queries: HashMap<String, Size>,
    styles: HashSet<String>,
}

impl Header {
    pub fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    pub fn set_title(&mut self, title: String) {
        self.title = Some(title);
    }

    pub fn breakpoint(&self) -> &Size {
        &self.breakpoint
    }

    pub fn set_breakpoint(&mut self, value: Size) {
        self.breakpoint = value;
    }

    pub fn has_media_queries(&self) -> bool {
        !self.media_queries.is_empty()
    }

    pub fn add_media_query<K: ToString>(&mut self, classname: K, size: Size) {
        self.media_queries.insert(classname.to_string(), size);
    }

    pub fn maybe_add_style<K: ToString>(&mut self, style: Option<K>) {
        match style {
            Some(value) => self.add_style(value),
            None => (),
        }
    }

    pub fn add_style<K: ToString>(&mut self, style: K) {
        self.styles.insert(style.to_string());
    }

    pub fn maybe_add_font_families(&mut self, font_family_list: Option<String>) {
        match font_family_list {
            Some(value) => self.add_font_families(value),
            None => (),
        }
    }

    pub fn add_font_families(&mut self, font_family_list: String) {
        let result = font_family_list
            .split(",")
            .map(|v| v.trim().to_string())
            .collect::<Vec<String>>();
        for item in result {
            self.font_families.insert(item);
        }
    }

    pub fn register_font(&mut self, name: &str, href: &str) {
        self.font_registry.add(name, href);
    }

    pub fn get_styles(&self) -> Vec<String> {
        self.styles.iter().cloned().collect()
    }

    pub fn get_font_families(&self) -> Vec<String> {
        self.font_families.iter().cloned().collect()
    }

    pub fn get_used_font_families(&self) -> Vec<&String> {
        let mut res = vec![];
        for name in self.font_families.iter() {
            if let Some(url) = self.font_registry.get(name) {
                res.push(url);
            }
        }
        res
    }

    pub fn get_media_queries(&self) -> &HashMap<String, Size> {
        &self.media_queries
    }
}

impl From<&Options> for Header {
    fn from(value: &Options) -> Self {
        Header {
            breakpoint: value.breakpoint.clone(),
            font_registry: value.fonts.clone(),
            font_families: HashSet::new(),
            title: None,
            media_queries: HashMap::new(),
            styles: HashSet::new(),
        }
    }
}
