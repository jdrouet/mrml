use super::prelude::PropertyMap;
use super::size::Size;
use crate::Options;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Context {
    inner: HashMap<String, String>,
    options: Options,
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
    pub fn default(options: Options) -> Self {
        Self::new(options, None, 1, 0, 0)
    }

    pub fn new(
        options: Options,
        container_width: Option<Size>,
        siblings: usize,
        raw_siblings: usize,
        index: usize,
    ) -> Self {
        Self {
            inner: HashMap::new(),
            options,
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
            options: other.options.clone(),
            inner: HashMap::new(),
            container_width,
            siblings,
            raw_siblings,
            index,
        };
        ctx.merge(other);
        ctx
    }

    pub fn options(&self) -> &Options {
        &self.options
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Options;

    #[test]
    fn with_default_from() {
        let origin = Context::default(Options::default());
        let result = Context::default_from(&origin);
        assert_eq!(origin.options().breakpoint, result.options().breakpoint);
        assert_eq!(
            origin.options().keep_comments,
            result.options().keep_comments
        );
    }

    #[test]
    fn set_container_width() {
        let mut ctx = Context::default(Options::default());
        ctx.set_container_width(Some(Size::Percent(32.0)));
        assert_eq!(ctx.container_width().unwrap().value(), 32.0);
    }

    #[test]
    fn with_params() {
        let mut ctx = Context::new(Options::default(), Some(Size::Percent(42.0)), 4, 2, 1);
        assert_eq!(ctx.container_width().unwrap().value(), 42.0);
        assert_eq!(ctx.siblings(), 4);
        ctx.set_siblings(6);
        assert_eq!(ctx.siblings(), 6);
        assert_eq!(ctx.raw_siblings(), 2);
        assert_eq!(ctx.non_raw_siblings(), 4);
        ctx.set_raw_siblings(5);
        assert_eq!(ctx.raw_siblings(), 5);
        assert_eq!(ctx.non_raw_siblings(), 1);
        assert_eq!(ctx.index(), 1);
        assert_eq!(ctx.is_first(), false);
        assert_eq!(ctx.is_last(), false);
        ctx.set_index(0);
        assert_eq!(ctx.is_first(), true);
        assert_eq!(ctx.is_last(), false);
        ctx.set_index(5);
        assert_eq!(ctx.is_first(), false);
        assert_eq!(ctx.is_last(), true);
    }
}
