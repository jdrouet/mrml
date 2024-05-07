use std::{borrow::Cow, fmt::Write};

use super::RenderBuffer;
use crate::prelude::hash::{Map, Set};

#[derive(Default)]
struct Styles<'a>(Vec<(Cow<'a, str>, Cow<'a, str>)>);

impl<'a> std::fmt::Debug for Styles<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('"')?;
        for (key, value) in self.0.iter() {
            write!(f, "{key}:{value};")?;
        }
        f.write_char('"')
    }
}

#[derive(Default)]
struct Classes<'a>(Set<Cow<'a, str>>);

impl<'a> std::fmt::Debug for Classes<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('"')?;
        for (i, c) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_char(' ')?;
            }
            f.write_str(c)?;
        }
        f.write_char('"')
    }
}

pub struct Tag<'a> {
    name: Cow<'a, str>,
    attributes: Map<Cow<'a, str>, Cow<'a, str>>,
    classes: Classes<'a>,
    // in order to keep the style in the same order the've been added
    styles: Styles<'a>,
}

impl<'a> Tag<'a> {
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

    pub fn new<N: Into<Cow<'a, str>>>(name: N) -> Self {
        Self {
            name: name.into(),
            attributes: Map::new(),
            classes: Default::default(),
            styles: Default::default(),
        }
    }

    pub fn add_class<C: Into<Cow<'a, str>>>(mut self, value: C) -> Self {
        self.classes.0.insert(value.into());
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

    pub fn maybe_add_class<C: Into<Cow<'a, str>>>(self, value: Option<C>) -> Self {
        if let Some(value) = value {
            self.add_class(value)
        } else {
            self
        }
    }

    pub fn add_attribute<K: Into<Cow<'a, str>>, V: Into<Cow<'a, str>>>(
        mut self,
        name: K,
        value: V,
    ) -> Self {
        self.attributes.insert(name.into(), value.into());
        self
    }

    pub fn maybe_add_attribute<K: Into<Cow<'a, str>>, V: Into<Cow<'a, str>>>(
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

    pub fn add_style<N: Into<Cow<'a, str>>, V: Into<Cow<'a, str>>>(
        mut self,
        name: N,
        value: V,
    ) -> Self {
        self.styles.0.push((name.into(), value.into()));
        self
    }

    pub fn maybe_add_style<N: Into<Cow<'a, str>>, V: Into<Cow<'a, str>>>(
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
}

impl<'a> Tag<'a> {
    fn render_opening(&self, b: &mut RenderBuffer) -> std::fmt::Result {
        b.push('<');
        b.push_str(&self.name);
        for (key, value) in self.attributes.iter() {
            b.push_attribute(key, value)?;
        }
        if !self.classes.0.is_empty() {
            b.push_attribute("class", &self.classes)?;
        }
        if !self.styles.0.is_empty() {
            b.push_attribute("style", &self.styles)?;
        }
        Ok(())
    }

    pub fn render_open(&self, b: &mut RenderBuffer) -> std::fmt::Result {
        self.render_opening(b)?;
        b.push('>');
        Ok(())
    }

    pub fn render_close(&self, b: &mut RenderBuffer) {
        b.push_str("</");
        b.push_str(self.name.as_ref());
        b.push('>');
    }

    pub fn render_closed(&self, b: &mut RenderBuffer) -> std::fmt::Result {
        self.render_opening(b)?;
        b.push_str(" />");
        Ok(())
    }

    pub fn render_with<F>(&self, buf: &mut RenderBuffer, cb: F) -> std::fmt::Result
    where
        F: FnOnce(&mut RenderBuffer) -> std::fmt::Result,
    {
        self.render_open(buf)?;
        cb(buf)?;
        self.render_close(buf);
        Ok(())
    }

    pub fn render_text(&self, buf: &mut RenderBuffer, value: &str) -> std::fmt::Result {
        self.render_open(buf)?;
        buf.push_str(value);
        self.render_close(buf);
        Ok(())
    }
}
