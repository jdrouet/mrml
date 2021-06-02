use super::{MJAttributes, MJAttributesChild, NAME};
use crate::print_children;

impl MJAttributesChild {
    fn as_print(&self) -> &dyn Print {
        match self {
            Self::MJAttributesAll(elt) => elt,
            Self::MJAttributesClass(elt) => elt,
            Self::MJAttributesElement(elt) => elt,
        }
    }
}

impl Print for MJAttributesChild {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        self.as_print().print(pretty, level, indent_size)
    }
}

print_children!(MJAttributes, NAME);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_attributes::MJAttributes::default();
        assert_eq!("<mj-attributes></mj-attributes>", item.dense_print());
    }
}
