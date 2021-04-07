use super::MJAccordionTitle;
use crate::prelude::print::{self, Print};
use std::fmt;

impl Print for MJAccordionTitle {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        print::open(
            super::NAME,
            Some(&self.attributes),
            false,
            pretty,
            level,
            indent_size,
        ) + &self
            .children
            .iter()
            .map(|child| child.print(pretty, level + 1, indent_size))
            .collect::<String>()
            + &print::close(super::NAME, pretty, level, indent_size)
    }
}

impl fmt::Display for MJAccordionTitle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.dense_print().as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_accordion_title::MJAccordionTitle::default();
        assert_eq!(
            "<mj-accordion-title></mj-accordion-title>",
            item.dense_print()
        );
    }
}
