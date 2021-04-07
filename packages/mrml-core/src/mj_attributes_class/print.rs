use super::MJAttributesClass;
use crate::prelude::print::{self, Print};
use std::fmt;

impl Print for MJAttributesClass {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        let mut attrs = self.attributes.clone();
        attrs.insert("name".to_string(), self.name.clone());
        print::open(super::NAME, Some(&attrs), true, pretty, level, indent_size)
    }
}

impl fmt::Display for MJAttributesClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.dense_print().as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_attributes_all::MJAttributesAll::default();
        assert_eq!("<mj-all />", item.dense_print());
    }
}
