use std::borrow::Cow;
use std::convert::TryFrom;
use std::sync::atomic::{AtomicU16, Ordering};

use crate::helper::size::{Pixel, Size};
use crate::mj_head::MjHead;
use crate::prelude::hash::Map;
use crate::prelude::hash::Set;

pub struct Header<'h> {
    head: &'h Option<MjHead>,
    attributes_all: Map<&'h str, &'h str>,
    attributes_class: Map<&'h str, Map<&'h str, &'h str>>,
    attributes_element: Map<&'h str, Map<&'h str, &'h str>>,
    breakpoint: Pixel,
    font_families: Map<&'h str, &'h str>,
    used_font_families: Set<String>,
    media_queries: Map<String, Size>,
    styles: Set<Cow<'static, str>>,
    lang: Option<String>,
    generator: AtomicU16,
}

impl<'h> Header<'h> {
    pub fn new(head: &'h Option<MjHead>) -> Self {
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
            used_font_families: Set::new(),
            media_queries: Map::new(),
            styles: Set::new(),
            lang: Default::default(),
            generator: AtomicU16::new(0),
        }
    }

    pub fn attribute_all(&self, key: &str) -> Option<&str> {
        self.attributes_all.get(key).copied()
    }

    pub fn attribute_class(&self, name: &str, key: &str) -> Option<&str> {
        self.attributes_class
            .get(name)
            .and_then(|class_map| class_map.get(key))
            .copied()
    }

    pub fn attribute_element(&self, name: &str, key: &str) -> Option<&str> {
        self.attributes_element
            .get(name)
            .and_then(|elt| elt.get(key))
            .copied()
    }

    pub fn head(&self) -> &Option<MjHead> {
        self.head
    }

    pub fn breakpoint(&self) -> &Pixel {
        &self.breakpoint
    }

    pub fn add_used_font_family(&mut self, value: &str) {
        self.used_font_families.insert(value.to_string());
    }

    pub fn add_font_families<T: AsRef<str>>(&mut self, value: T) {
        for name in value
            .as_ref()
            .split(',')
            .map(|item| item.trim())
            .filter(|item| !item.is_empty())
        {
            self.add_used_font_family(name);
        }
    }

    pub fn maybe_add_font_families<T: AsRef<str>>(&mut self, value: Option<T>) {
        if let Some(value) = value {
            self.add_font_families(value);
        }
    }

    pub fn used_font_families(&self) -> &Set<String> {
        &self.used_font_families
    }

    pub fn font_families(&self) -> &Map<&str, &str> {
        &self.font_families
    }

    pub fn media_queries(&self) -> &Map<String, Size> {
        &self.media_queries
    }

    pub fn add_media_query(&mut self, classname: String, size: Size) {
        self.media_queries.insert(classname, size);
    }

    pub fn styles(&self) -> &Set<Cow<'static, str>> {
        &self.styles
    }

    pub fn add_style<V: Into<Cow<'static, str>>>(&mut self, value: V) {
        self.styles.insert(value.into());
    }

    pub fn maybe_add_style<V: Into<Cow<'static, str>>>(&mut self, value: Option<V>) {
        if let Some(value) = value {
            self.add_style(value);
        }
    }

    pub fn lang(&self) -> Option<&str> {
        self.lang.as_deref()
    }

    pub fn maybe_set_lang(&mut self, value: Option<String>) {
        self.lang = value;
    }

    pub fn next_id(&self) -> String {
        let id = self.generator.fetch_add(1, Ordering::SeqCst);
        format!("{id:0>8}")
    }
}
