use super::MJButton;
use crate::prelude::print::{self, Print};
use std::fmt;

impl Print for MJButton {
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

impl fmt::Display for MJButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.dense_print().as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let mut item = crate::mj_button::MJButton::default();
        item.attributes
            .insert("href".to_string(), "http://localhost".into());
        assert_eq!(
            "<mj-button href=\"http://localhost\"></mj-button>",
            item.dense_print()
        );
        assert_eq!(
            "<mj-button href=\"http://localhost\">\n</mj-button>\n",
            item.pretty_print()
        );
    }
}
