use super::MJDivider;
use crate::prelude::print::{self, Print};
use std::fmt;

impl Print for MJDivider {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        print::open(
            super::NAME,
            Some(&self.attributes),
            true,
            pretty,
            level,
            indent_size,
        )
    }
}

impl fmt::Display for MJDivider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.dense_print().as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let mut item = crate::mj_divider::MJDivider::default();
        item.attributes
            .insert("src".to_string(), "http://localhost".into());
        assert_eq!(
            "<mj-divider src=\"http://localhost\" />",
            item.dense_print()
        );
    }
}
