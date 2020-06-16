use super::attributes::{Attributes, Merge};
use super::fonts::FontRegistry;
use super::Size;
use crate::Options;
use roxmltree::Node;
use std::collections::HashMap;
use std::collections::HashSet;
use std::string::ToString;

#[derive(Clone, Debug)]
pub struct DefaultAttributes {
    all: Attributes,
    classes: HashMap<String, Attributes>,
    elements: HashMap<String, Attributes>,
}

impl DefaultAttributes {
    pub fn new() -> Self {
        Self {
            all: Attributes::new(),
            classes: HashMap::new(),
            elements: HashMap::new(),
        }
    }

    pub fn add_all_content<K, V, I>(&mut self, items: I)
    where
        K: ToString,
        V: ToString,
        I: Iterator<Item = (K, V)>,
    {
        self.all.merge_iter(items);
    }

    fn get_mut_element(&mut self, name: &str) -> &mut Attributes {
        if !self.elements.contains_key(name) {
            self.elements.insert(name.to_string(), Attributes::new());
        }
        self.elements.get_mut(name).unwrap()
    }

    pub fn add_element_content<K, V, I>(&mut self, name: &str, items: I)
    where
        K: ToString,
        V: ToString,
        I: Iterator<Item = (K, V)>,
    {
        let container = self.get_mut_element(name);
        for (key, value) in items {
            container.set(key, value);
        }
    }

    fn get_mut_class(&mut self, name: &str) -> &mut Attributes {
        if !self.classes.contains_key(name) {
            self.classes.insert(name.to_string(), Attributes::new());
        }
        self.classes.get_mut(name).unwrap()
    }

    pub fn add_class_content<K, V, I>(&mut self, name: &str, items: I)
    where
        K: ToString,
        V: ToString,
        I: Iterator<Item = (K, V)>,
    {
        let container = self.get_mut_class(name);
        for (key, value) in items {
            container.set(key, value);
        }
    }

    pub fn set_element_attributes(&self, name: &str, other: Attributes) -> Attributes {
        let result = other.concat(&self.all);
        match self.elements.get(name) {
            Some(attrs) => result.concat(attrs),
            None => result,
        }
    }

    pub fn set_class_attributes(&self, name: &str, other: Attributes) -> Attributes {
        match self.classes.get(name) {
            Some(attrs) => other.concat(attrs),
            None => other,
        }
    }

    pub fn get_attributes<'a, 'b>(&self, node: &Node<'a, 'b>, base: Attributes) -> Attributes {
        let tag_name = node.tag_name().name();
        let mut result = base.concat(&self.all);
        if let Some(element) = self.elements.get(tag_name) {
            result.merge(element);
        }
        if let Some(classes) = node
            .attribute("mj-class")
            .and_then(|value| Some(value.split(" ").collect::<Vec<&str>>()))
        {
            for classname in classes {
                if let Some(attrs) = self.classes.get(classname) {
                    result.merge(attrs);
                }
            }
        }
        result
    }
}

#[derive(Clone, Debug)]
pub struct Header {
    breakpoint: Size,
    default_attributes: DefaultAttributes,
    font_families: HashSet<String>,
    font_registry: FontRegistry,
    keep_comments: bool,
    media_queries: HashMap<String, Size>,
    preview: Option<String>,
    title: Option<String>,
    styles: HashSet<String>,
}

impl Header {
    pub fn keep_comments(&self) -> bool {
        self.keep_comments
    }

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

    pub fn default_attributes(&self) -> &DefaultAttributes {
        &self.default_attributes
    }

    pub fn set_default_attributes(&mut self, value: DefaultAttributes) {
        self.default_attributes = value;
    }

    pub fn preview(&self) -> Option<&String> {
        self.preview.as_ref()
    }

    pub fn set_preview(&mut self, preview: String) {
        self.preview = Some(preview);
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

    pub fn maybe_add_font_families(&mut self, font_family_list: Option<&String>) {
        match font_family_list {
            Some(value) => self.add_font_families(value),
            None => (),
        }
    }

    pub fn add_font_families(&mut self, font_family_list: &String) {
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
            default_attributes: DefaultAttributes::new(),
            font_families: HashSet::new(),
            font_registry: value.fonts.clone(),
            keep_comments: value.keep_comments,
            media_queries: HashMap::new(),
            preview: None,
            title: None,
            styles: HashSet::new(),
        }
    }
}
