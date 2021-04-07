use super::MJAttributesElement;
use crate::prelude::print::{self, Print};
use std::fmt;

impl Print for MJAttributesElement {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        print::open(
            &self.name,
            Some(&self.attributes),
            true,
            pretty,
            level,
            indent_size,
        )
    }
}

impl fmt::Display for MJAttributesElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.dense_print().as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_attributes_element::MJAttributesElement::new("span".to_string());
        assert_eq!("<span />", item.dense_print());
    }
}
