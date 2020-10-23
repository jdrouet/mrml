use super::attributes::{Attributes, Merge};
use super::fonts::FontRegistry;
use super::size::Size;
use crate::parser::Node;
use crate::Options;
use std::collections::HashMap;
use std::collections::HashSet;
use std::string::ToString;

#[derive(Clone, Debug, Default)]
pub struct DefaultAttributes {
    all: Attributes,
    classes: HashMap<String, Attributes>,
    elements: HashMap<String, Attributes>,
}

impl DefaultAttributes {
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
            self.elements
                .insert(name.to_string(), Attributes::default());
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
            self.classes.insert(name.to_string(), Attributes::default());
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

    pub fn get_attributes<'a>(&self, node: &Node<'a>, base: Attributes) -> Attributes {
        let tag_name = node.name.as_str();
        let mut result = base.concat(&self.all);
        if let Some(element) = self.elements.get(tag_name) {
            result.merge(element);
        }
        if let Some(classes) = node
            .attributes
            .iter()
            .find(|(key, _value)| key.as_str() == "mj-class")
            .map(|(_key, value)| value.as_str().split(' ').collect::<Vec<&str>>())
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
    pub breakpoint: Size,
    pub default_attributes: DefaultAttributes,
    pub font_families: HashSet<String>,
    pub font_registry: FontRegistry,
    pub keep_comments: bool,
    pub media_queries: HashMap<String, Size>,
    pub preview: Option<String>,
    pub title: Option<String>,
    pub styles: HashSet<String>,
    pub social_icon_origin: String,
}

impl Header {
    pub fn set_title(&mut self, title: String) {
        self.title = Some(title);
    }

    pub fn set_breakpoint(&mut self, value: Size) {
        self.breakpoint = value;
    }

    pub fn set_default_attributes(&mut self, value: DefaultAttributes) {
        self.default_attributes = value;
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
        if let Some(value) = style {
            self.add_style(value);
        }
    }

    pub fn add_style<K: ToString>(&mut self, style: K) {
        self.styles.insert(style.to_string());
    }

    pub fn maybe_add_font_families(&mut self, font_family_list: Option<&String>) {
        if let Some(value) = font_family_list {
            self.add_font_families(value);
        }
    }

    pub fn add_font_families(&mut self, font_family_list: &str) {
        let result = font_family_list
            .split(',')
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
}

impl From<Options> for Header {
    fn from(value: Options) -> Self {
        Header {
            breakpoint: value.breakpoint.clone(),
            default_attributes: DefaultAttributes::default(),
            font_families: HashSet::new(),
            font_registry: value.fonts.clone(),
            keep_comments: value.keep_comments,
            media_queries: HashMap::new(),
            preview: None,
            title: None,
            styles: HashSet::new(),
            social_icon_origin: value.social_icon_origin,
        }
    }
}
