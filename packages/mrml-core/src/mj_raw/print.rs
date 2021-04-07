use super::MJRaw;
use crate::prelude::print::{self, Print};
use std::fmt;

impl Print for MJRaw {
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

impl fmt::Display for MJRaw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.dense_print().as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_raw::MJRaw::default();
        assert_eq!("<mj-raw></mj-raw>", item.dense_print());
    }
}
