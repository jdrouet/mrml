use super::MJAttributesClass;
use crate::prelude::print::{self, Print};
use crate::print_display;

impl Print for MJAttributesClass {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        let mut attrs = self.attributes.clone();
        attrs.insert("name".to_string(), self.name.clone());
        print::open(super::NAME, Some(&attrs), true, pretty, level, indent_size)
    }
}

print_display!(MJAttributesClass);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_attributes_all::MJAttributesAll::default();
        assert_eq!("<mj-all />", item.dense_print());
    }
}
