use super::Size;
use std::collections::HashMap;
use std::collections::HashSet;
use std::string::ToString;

#[derive(Clone, Debug)]
pub struct Header {
    title: Option<String>,
    media_queries: HashMap<String, Size>,
    styles: HashSet<String>,
    font_families: HashSet<String>,
}

impl Header {
    pub fn new() -> Self {
        Header {
            title: None,
            media_queries: HashMap::new(),
            styles: HashSet::new(),
            font_families: HashSet::new(),
        }
    }

    pub fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    pub fn set_title(&mut self, title: String) {
        self.title = Some(title);
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

    pub fn get_styles(&self) -> Vec<String> {
        self.styles.iter().cloned().collect()
    }

    pub fn get_font_families(&self) -> Vec<String> {
        self.font_families.iter().cloned().collect()
    }

    pub fn get_media_queries(&self) -> &HashMap<String, Size> {
        &self.media_queries
    }
}
