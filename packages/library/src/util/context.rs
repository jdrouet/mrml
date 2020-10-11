use super::size::Size;

#[derive(Clone, Debug)]
pub struct Context {
    container_width: Option<Size>,
    siblings: usize,
    raw_siblings: usize,
    index: usize,
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
            container_width,
            siblings,
            raw_siblings,
            index,
        }
    }

    pub fn from(other: &Self) -> Self {
        Self {
            container_width: other.container_width(),
            siblings: other.siblings(),
            raw_siblings: other.raw_siblings(),
            index: other.index(),
        }
    }

    pub fn container_width(&self) -> Option<Size> {
        self.container_width.clone()
    }

    pub fn set_container_width(mut self, value: Option<Size>) -> Self {
        self.container_width = value;
        self
    }

    pub fn siblings(&self) -> usize {
        self.siblings
    }

    pub fn set_siblings(mut self, value: usize) -> Self {
        self.siblings = value;
        self
    }

    pub fn raw_siblings(&self) -> usize {
        self.raw_siblings
    }

    pub fn set_raw_siblings(mut self, value: usize) -> Self {
        self.raw_siblings = value;
        self
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

    #[test]
    fn set_container_width() {
        let ctx = Context::default().set_container_width(Some(Size::Percent(32.0)));
        assert_eq!(ctx.container_width().unwrap().value(), 32.0);
    }

    #[test]
    fn with_params() {
        let ctx = Context::new(Some(Size::Percent(42.0)), 4, 2, 1);
        assert_eq!(ctx.container_width().unwrap().value(), 42.0);
        assert_eq!(ctx.siblings(), 4);
        let ctx = ctx.set_siblings(6);
        assert_eq!(ctx.siblings(), 6);
        assert_eq!(ctx.raw_siblings(), 2);
        assert_eq!(ctx.non_raw_siblings(), 4);
        let ctx = ctx.set_raw_siblings(5);
        assert_eq!(ctx.raw_siblings(), 5);
        assert_eq!(ctx.non_raw_siblings(), 1);
        assert_eq!(ctx.index(), 1);
        assert_eq!(ctx.is_first(), false);
        assert_eq!(ctx.is_last(), false);
        let ctx = ctx.set_index(0);
        assert_eq!(ctx.is_first(), true);
        assert_eq!(ctx.is_last(), false);
        let ctx = ctx.set_index(5);
        assert_eq!(ctx.is_first(), false);
        assert_eq!(ctx.is_last(), true);
    }
}
