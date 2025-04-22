use std::borrow::Cow;
use std::fmt::Write;

use super::RenderBuffer;
use crate::prelude::hash::{Map, Set};

#[derive(Default)]
pub(super) struct Styles<'a>(Vec<(Cow<'a, str>, Cow<'a, str>)>);

impl std::fmt::Debug for Styles<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('"')?;
        for (key, value) in self.0.iter() {
            write!(f, "{key}:{value};")?;
        }
        f.write_char('"')
    }
}

#[derive(Default)]
pub(super) struct Classes<'a>(Set<Cow<'a, str>>);

impl std::fmt::Debug for Classes<'_> {
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

pub fn matches_selector(tag: &Tag, selector: &str) -> bool {
    let mut tag_name: Option<&str> = None;
    let mut class_selector: Option<&str> = None;
    let mut id_selector: Option<&str> = None;
    let mut attribute_selector: Option<&str> = None;

    let mut current = selector;

    if !current.starts_with('.') && !current.starts_with('#') && !current.starts_with('[') {
        if let Some(first_special) = current.find(|c| c == '.' || c == '#' || c == '[') {
            tag_name = Some(&current[..first_special]);
            current = &current[first_special..];
        } else {
            tag_name = Some(current);
            current = "";
        }
    }

    while !current.is_empty() {
        if current.starts_with('.') {
            if class_selector.is_none() {
                if let Some(next_special) = current[1..].find(|c| c == '.' || c == '#' || c == '[')
                {
                    class_selector = Some(&current[..next_special + 1]);
                    current = &current[next_special + 1..];
                } else {
                    class_selector = Some(current);
                    current = "";
                }
            } else {
                break;
            }
        } else if current.starts_with('#') {
            if id_selector.is_none() {
                if let Some(next_special) = current[1..].find(|c| c == '.' || c == '#' || c == '[')
                {
                    id_selector = Some(&current[..next_special + 1]);
                    current = &current[next_special + 1..];
                } else {
                    id_selector = Some(current);
                    current = "";
                }
            } else {
                break;
            }
        } else if current.starts_with('[') {
            if attribute_selector.is_none() {
                if let Some(end_bracket) = current.find(']') {
                    attribute_selector = Some(&current[..end_bracket + 1]);
                    current = &current[end_bracket + 1..];
                } else {
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    let tag_name_matches = tag_name.map_or(true, |name| tag.name.as_ref() == name);

    let class_matches = class_selector.map_or(true, |class_sel| {
        let class_name = &class_sel[1..];
        tag.classes.0.iter().any(|c| c.as_ref() == class_name)
    });

    let id_matches = id_selector.map_or(true, |id_sel| {
        let id_name = &id_sel[1..];
        tag.attributes
            .get("id")
            .map(|id| id.as_ref() == id_name)
            .unwrap_or(false)
    });

    let attribute_matches = attribute_selector.map_or(true, |attr_sel| {
        if attr_sel.starts_with('[') && attr_sel.ends_with(']') {
            let content = &attr_sel[1..attr_sel.len() - 1];
            if let Some((attr_name, attr_value)) = content.split_once('=') {
                let value = attr_value.trim_matches('"').trim_matches('\'');
                tag.attributes
                    .get(attr_name)
                    .map(|v| v.as_ref() == value)
                    .unwrap_or(false)
            } else {
                tag.attributes.contains_key(content)
            }
        } else {
            true
        }
    });

    tag_name_matches && class_matches && id_matches && attribute_matches
}

pub(crate) struct Tag<'a> {
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

    pub fn set_html_attributes(
        mut self,
        html_attributes: &Map<&'a str, Map<&'a str, &'a str>>,
    ) -> Self {
        for (selector, attrs) in html_attributes.iter() {
            if matches_selector(&self, selector) {
                for (name, value) in attrs.iter() {
                    self.attributes.insert(
                        std::borrow::Cow::from(*name),
                        std::borrow::Cow::from(*value),
                    );
                }
            }
        }
        self
    }
}

impl Tag<'_> {
    fn render_opening(&self, b: &mut RenderBuffer) -> std::fmt::Result {
        b.push('<');
        b.push_str(&self.name);
        for (key, value) in self.attributes.iter() {
            b.push_attribute(key.as_ref(), value.as_ref())?;
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
