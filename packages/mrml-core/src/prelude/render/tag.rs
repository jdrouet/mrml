use std::borrow::Cow;

use super::RenderBuffer;
use crate::prelude::hash::{Map, Set};

pub struct Tag {
    name: Cow<'static, str>,
    attributes: Map<Cow<'static, str>, Cow<'static, str>>,
    classes: Set<Cow<'static, str>>,
    // in order to keep the style in the same order the've been added
    styles: Vec<(Cow<'static, str>, Cow<'static, str>)>,
}

impl Tag {
    pub fn table() -> Self {
        Self::new("table")
    }
    pub fn table_borderless() -> Self {
        Self::table()
            .add_attribute("border", "0")
            .add_attribute("cellpadding", "0")
            .add_attribute("cellspacing", "0")
    }
    pub fn table_presentation() -> Self {
        Self::table_borderless().add_attribute("role", "presentation")
    }
    pub fn tbody() -> Self {
        Self::new("tbody")
    }
    pub fn tr() -> Self {
        Self::new("tr")
    }
    pub fn td() -> Self {
        Self::new("td")
    }
    pub fn div() -> Self {
        Self::new("div")
    }

    pub fn new<N: Into<Cow<'static, str>>>(name: N) -> Self {
        Self {
            name: name.into(),
            attributes: Map::new(),
            classes: Set::new(),
            styles: Vec::new(),
        }
    }

    pub fn add_class<C: Into<Cow<'static, str>>>(mut self, value: C) -> Self {
        self.classes.insert(value.into());
        self
    }

    pub fn add_suffixed_class<T: AsRef<str>>(self, value: T, suffix: &str) -> Self {
        self.add_class(format!("{}-{suffix}", value.as_ref()))
    }

    pub fn maybe_add_suffixed_class<T: AsRef<str>>(self, value: Option<T>, suffix: &str) -> Self {
        if let Some(value) = value {
            self.add_suffixed_class(value, suffix)
        } else {
            self
        }
    }

    pub fn maybe_add_class<C: Into<Cow<'static, str>>>(self, value: Option<C>) -> Self {
        if let Some(value) = value {
            self.add_class(value)
        } else {
            self
        }
    }

    pub fn add_attribute<K: Into<Cow<'static, str>>, V: Into<Cow<'static, str>>>(
        mut self,
        name: K,
        value: V,
    ) -> Self {
        self.attributes.insert(name.into(), value.into());
        self
    }

    pub fn maybe_add_attribute<K: Into<Cow<'static, str>>, V: Into<Cow<'static, str>>>(
        self,
        name: K,
        value: Option<V>,
    ) -> Self {
        if let Some(value) = value {
            self.add_attribute(name, value)
        } else {
            self
        }
    }

    pub fn add_style<N: Into<Cow<'static, str>>, V: Into<Cow<'static, str>>>(
        mut self,
        name: N,
        value: V,
    ) -> Self {
        self.styles.push((name.into(), value.into()));
        self
    }

    pub fn maybe_add_style<N: Into<Cow<'static, str>>, V: Into<Cow<'static, str>>>(
        self,
        name: N,
        value: Option<V>,
    ) -> Self {
        if let Some(value) = value {
            self.add_style(name, value)
        } else {
            self
        }
    }

    fn opening(&self) -> String {
        let mut res = String::new();
        self.render_opening(&mut res);
        res
    }

    fn render_opening(&self, b: &mut RenderBuffer) {
        b.push('<');
        b.push_str(&self.name);
        for (key, value) in self.attributes.iter() {
            b.push(' ');
            b.push_str(key);
            b.push_str("=\"");
            b.push_str(value);
            b.push('"');
        }
        if !self.classes.is_empty() {
            b.push_str(" class=\"");
            for (index, classname) in self.classes.iter().enumerate() {
                if index > 0 {
                    b.push(' ');
                }
                b.push_str(classname);
            }
            b.push('"');
        }
        if !self.styles.is_empty() {
            b.push_str(" style=\"");
            for (key, value) in self.styles.iter() {
                b.push_str(key);
                b.push(':');
                b.push_str(value);
                b.push(';');
            }
            b.push('"');
        }
    }

    pub fn open(&self) -> String {
        let mut buffer = String::new();
        self.render_open(&mut buffer);
        buffer
    }

    pub fn render_open(&self, b: &mut RenderBuffer) {
        self.render_opening(b);
        b.push('>');
    }

    pub fn close(&self) -> String {
        format!("</{}>", self.name)
    }

    pub fn render_close(&self, b: &mut RenderBuffer) {
        b.push_str("</");
        b.push_str(self.name.as_ref());
        b.push('>');
    }

    pub fn closed(&self) -> String {
        self.opening() + " />"
    }

    pub fn render_closed(&self, b: &mut RenderBuffer) {
        self.render_opening(b);
        b.push_str(" />");
    }

    pub fn render_with<F>(&self, buf: &mut RenderBuffer, cb: F)
    where
        F: FnOnce(&mut RenderBuffer),
    {
        self.render_open(buf);
        cb(buf);
        self.render_close(buf);
    }

    pub fn render_text(&self, buf: &mut RenderBuffer, value: &str) {
        self.render_open(buf);
        buf.push_str(value);
        self.render_close(buf);
    }

    pub fn render<T: AsRef<str>>(&self, input: T) -> String {
        let mut buf = String::default();
        self.render_open(&mut buf);
        buf.push_str(input.as_ref());
        self.render_close(&mut buf);
        buf
    }
}
