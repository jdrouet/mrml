use super::MJAttributes;
use crate::prelude::print::{self, Print};
use std::fmt;

impl Print for MJAttributes {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        print::open(super::NAME, None, false, pretty, level, indent_size)
            + &self
                .children
                .iter()
                .map(|child| child.print(pretty, level + 1, indent_size))
                .collect::<String>()
            + &print::close(super::NAME, pretty, level, indent_size)
    }
}

impl fmt::Display for MJAttributes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.dense_print().as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_attributes::MJAttributes::default();
        assert_eq!("<mj-attributes></mj-attributes>", item.dense_print());
    }
}
