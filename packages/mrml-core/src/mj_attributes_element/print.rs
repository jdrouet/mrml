use super::MJAttributesElement;
use crate::prelude::print::{self, Print};
use crate::print_display;

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

print_display!(MJAttributesElement);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_attributes_element::MJAttributesElement::new("span".to_string());
        assert_eq!("<span />", item.dense_print());
    }
}
