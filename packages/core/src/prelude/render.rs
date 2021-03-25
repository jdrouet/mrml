use crate::helper::size::{Pixel, Size};
use crate::mj_head::MJHead;
use std::cell::{Ref, RefCell};
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::rc::Rc;

#[derive(Debug)]
pub enum Error {}

pub struct Header<'h> {
    head: &'h Option<MJHead>,
    attributes_all: HashMap<&'h str, &'h str>,
    attributes_class: HashMap<&'h str, HashMap<&'h str, &'h str>>,
    attributes_element: HashMap<&'h str, HashMap<&'h str, &'h str>>,
    breakpoint: Pixel,
    font_families: HashMap<&'h str, &'h str>,
    used_font_families: HashSet<String>,
    media_queries: HashMap<String, Size>,
    styles: HashSet<String>,
}

impl<'h> Header<'h> {
    pub fn new(head: &'h Option<MJHead>) -> Self {
        Self {
            head,
            attributes_all: head
                .as_ref()
                .map(|h| h.build_attributes_all())
                .unwrap_or_default(),
            attributes_class: head
                .as_ref()
                .map(|h| h.build_attributes_class())
                .unwrap_or_default(),
            attributes_element: head
                .as_ref()
                .map(|h| h.build_attributes_element())
                .unwrap_or_default(),
            breakpoint: head
                .as_ref()
                .and_then(|h| h.breakpoint())
                .and_then(|s| Pixel::try_from(s.value()).ok())
                .unwrap_or_else(|| Pixel::new(480.0)),
            font_families: head
                .as_ref()
                .map(|h| h.build_font_families())
                .unwrap_or_default(),
            used_font_families: HashSet::new(),
            media_queries: HashMap::new(),
            styles: HashSet::new(),
        }
    }

    pub fn attribute_all(&self, key: &str) -> Option<&str> {
        self.attributes_all.get(key).map(|value| *value)
    }

    pub fn attribute_class(&self, name: &str, key: &str) -> Option<&str> {
        self.attributes_class
            .get(name)
            .and_then(|class_map| class_map.get(key))
            .map(|value| *value)
    }

    pub fn attribute_element(&self, name: &str, key: &str) -> Option<&str> {
        self.attributes_element
            .get(name)
            .and_then(|elt| elt.get(key))
            .map(|value| *value)
    }

    pub fn head(&self) -> &Option<MJHead> {
        &self.head
    }

    pub fn breakpoint(&self) -> &Pixel {
        &self.breakpoint
    }

    pub fn add_used_font_family(&mut self, value: &str) {
        self.used_font_families.insert(value.to_string());
    }

    pub fn used_font_families(&self) -> &HashSet<String> {
        &self.used_font_families
    }

    pub fn font_families(&self) -> &HashMap<&str, &str> {
        &self.font_families
    }

    pub fn media_queries(&self) -> &HashMap<String, Size> {
        &self.media_queries
    }

    pub fn add_media_query(&mut self, classname: String, size: Size) {
        self.media_queries.insert(classname, size);
    }

    pub fn styles(&self) -> &HashSet<String> {
        &self.styles
    }
}

pub trait Render<'h> {
    fn header(&self) -> Ref<Header<'h>>;
    fn tag(&self) -> Option<&str> {
        None
    }
    fn attributes(&self) -> Option<&HashMap<String, String>> {
        None
    }

    fn attribute(&self, key: &str) -> Option<String> {
        if let Some(value) = self.attributes().and_then(|attrs| attrs.get(key)) {
            return Some(value.clone());
        }
        let header = self.header();
        if let Some(tag) = self.tag() {
            if let Some(value) = header.attribute_element(tag, key) {
                return Some(value.to_string());
            }
        }
        // TODO handle classes
        header.attribute_all(key).map(|item| item.to_string())
    }

    fn set_index(&mut self, _index: usize) {}
    fn set_siblings(&mut self, _count: usize) {}
    fn set_raw_siblings(&mut self, _count: usize) {}

    fn render(&self, buf: &mut String) -> Result<(), Error>;
}

pub trait Renderable<'r, 'e: 'r, 'h: 'r> {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r>;
}
