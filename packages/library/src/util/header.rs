use super::attributes::{Attributes, Merge};
use super::fonts::FontRegistry;
use super::id::Generator as IdGenerator;
use super::size::Size;
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
        self.classes
            .entry(name.into())
            .or_insert_with(Attributes::default)
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

    pub fn concat_attributes(
        &self,
        tag: &str,
        default_attributes: &Attributes,
        attributes: &Attributes,
    ) -> Attributes {
        let mut result = default_attributes.clone().concat(&self.all);
        if let Some(elt) = self.elements.get(tag) {
            result.merge(elt);
        }
        let classes = attributes
            .get("mj-class")
            .map(|v| v.split(' ').map(String::from).collect::<Vec<String>>())
            .unwrap_or_default();
        classes
            .iter()
            .filter_map(|cname| self.classes.get(cname))
            .for_each(|attrs| result.merge(attrs));
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
    pub id_generator: IdGenerator,
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

    pub fn add_style<K: ToString>(&mut self, style: K) {
        self.styles.insert(style.to_string());
    }

    pub fn maybe_add_font_families(&mut self, font_family_list: Option<&String>) {
        if let Some(value) = font_family_list {
            self.add_font_families(value);
        }
    }

    pub fn add_font_families(&mut self, font_family_list: &str) {
        font_family_list
            .split(',')
            .map(|v| v.trim().to_string())
            .for_each(|item| {
                self.font_families.insert(item);
            });
    }

    pub fn register_font(&mut self, name: &str, href: &str) {
        self.font_registry.add(name, href);
    }

    pub fn get_styles(&self) -> Vec<String> {
        self.styles.iter().cloned().collect()
    }

    pub fn get_used_font_families(&self) -> Vec<&String> {
        self.font_families
            .iter()
            .filter_map(|name| self.font_registry.get(name))
            .collect()
    }
}

impl From<&Options> for Header {
    fn from(value: &Options) -> Self {
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
            social_icon_origin: value.social_icon_origin.clone(),
            id_generator: value.id_generator,
        }
    }
}
