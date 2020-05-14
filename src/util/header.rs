use super::Size;
use std::collections::HashMap;
use std::string::ToString;

#[derive(Clone, Debug)]
pub struct Header {
    title: Option<String>,
    media_queries: HashMap<String, Size>,
}

impl Header {
    pub fn new() -> Self {
        Header {
            title: None,
            media_queries: HashMap::new(),
        }
    }

    pub fn from(other: &Self) -> Self {
        let mut header = Header::new();
        header.merge(other);
        header
    }

    pub fn merge(&mut self, other: &Self) {
        if self.title.is_none() && other.title.is_some() {
            self.title = other.title.clone();
        }
        for (key, value) in other.media_queries.iter() {
            self.media_queries.insert(key.clone(), value.clone());
        }
    }

    pub fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    pub fn has_media_queries(&self) -> bool {
        !self.media_queries.is_empty()
    }

    pub fn add_media_query<K: ToString>(&mut self, classname: K, size: Size) {
        self.media_queries.insert(classname.to_string(), size);
    }

    pub fn get_media_queries(&self) -> &HashMap<String, Size> {
        &self.media_queries
    }
}
