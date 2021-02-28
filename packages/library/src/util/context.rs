use super::size::Size;

#[derive(Clone, Debug)]
pub struct Context {
    container_width: Option<Size>,
    siblings: usize,
    raw_siblings: usize,
    index: usize,
}

impl Default for Context {
    fn default() -> Self {
        Self::new(None, 1, 0, 0)
    }
}

impl Context {
    pub fn new(
        container_width: Option<Size>,
        siblings: usize,
        raw_siblings: usize,
        index: usize,
    ) -> Self {
        Self {
            container_width,
            siblings,
            raw_siblings,
            index,
        }
    }

    pub fn container_width(&self) -> Option<Size> {
        self.container_width.clone()
    }

    pub fn non_raw_siblings(&self) -> usize {
        self.siblings - self.raw_siblings
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn set_index(mut self, value: usize) -> Self {
        self.index = value;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_container_width() {
        let ctx = Context {
            container_width: Some(Size::Percent(32.0)),
            ..Context::default()
        };
        assert_eq!(ctx.container_width().unwrap().value(), 32.0);
    }

    #[test]
    fn with_params() {
        let ctx = Context::new(Some(Size::Percent(42.0)), 4, 2, 1);
        assert_eq!(ctx.container_width().unwrap().value(), 42.0);
        assert_eq!(ctx.non_raw_siblings(), 2);
        assert_eq!(ctx.index(), 1);
    }
}
