use super::{MJRaw, MJRawChild};
use crate::print_children;

impl MJRawChild {
    pub fn as_print<'p>(&'p self) -> &'p (dyn Print + 'p) {
        match self {
            Self::Comment(elt) => elt,
            Self::Node(elt) => elt,
            Self::Text(elt) => elt,
        }
    }
}

impl Print for MJRawChild {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        self.as_print().print(pretty, level, indent_size)
    }
}

print_children!(MJRaw, super::NAME);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_raw::MJRaw::default();
        assert_eq!("<mj-raw></mj-raw>", item.dense_print());
    }
}
