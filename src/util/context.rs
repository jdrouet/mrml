use super::prelude::PropertyMap;
use super::size::Size;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Context {
    inner: HashMap<String, String>,
    container_width: Option<Size>,
    siblings: usize,
    raw_siblings: usize,
    index: usize,
}

impl PropertyMap for Context {
    fn inner(&self) -> &HashMap<String, String> {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.inner
    }
}

impl Context {
    pub fn default() -> Self {
        Self::new(None, 1, 0, 0)
    }

    pub fn new(
        container_width: Option<Size>,
        siblings: usize,
        raw_siblings: usize,
        index: usize,
    ) -> Self {
        Self {
            inner: HashMap::new(),
            container_width,
            siblings,
            raw_siblings,
            index,
        }
    }

    pub fn default_from(other: &Self) -> Self {
        Self::from(other, None, 1, 0, 0)
    }

    pub fn from(
        other: &Self,
        container_width: Option<Size>,
        siblings: usize,
        raw_siblings: usize,
        index: usize,
    ) -> Self {
        let mut ctx = Self {
            inner: HashMap::new(),
            container_width,
            siblings,
            raw_siblings,
            index,
        };
        ctx.merge(other);
        ctx
    }

    pub fn container_width(&self) -> Option<Size> {
        self.container_width.clone()
    }

    pub fn set_container_width(&mut self, value: Option<Size>) {
        self.container_width = value;
    }

    pub fn siblings(&self) -> usize {
        self.siblings
    }

    pub fn set_siblings(&mut self, value: usize) {
        self.siblings = value;
    }

    pub fn raw_siblings(&self) -> usize {
        self.raw_siblings
    }

    pub fn set_raw_siblings(&mut self, value: usize) {
        self.raw_siblings = value;
    }

    pub fn non_raw_siblings(&self) -> usize {
        self.siblings - self.raw_siblings
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn set_index(&mut self, value: usize) {
        self.index = value;
    }

    pub fn is_first(&self) -> bool {
        self.index == 0
    }

    pub fn is_last(&self) -> bool {
        self.index == self.siblings - 1
    }
}
