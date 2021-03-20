use super::MJTitle;
use crate::prelude::print::{print_close, print_indent, print_open, Print};
use std::fmt;

impl Print for MJTitle {
    fn print(&self, f: &mut String, pretty: bool, level: usize, indent_size: usize) {
        print_open(f, super::NAME, None, false, pretty, level, indent_size);
        if pretty {
            print_indent(f, level + 1, indent_size);
        }
        f.push_str(self.0.as_str());
        if pretty {
            f.push_str("\n");
        }
        print_close(f, super::NAME, pretty, level, indent_size);
    }
}

impl fmt::Display for MJTitle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.dense_print().as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn normal() {
        let item = crate::mj_title::MJTitle::from("Hello World!");
        assert_eq!("<mj-title>Hello World!</mj-title>", item.dense_print());
        assert_eq!(
            "<mj-title>\n  Hello World!\n</mj-title>\n",
            item.pretty_print()
        );
    }
}
