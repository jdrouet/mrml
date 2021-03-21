use super::MJPreview;
use crate::prelude::print::{print_close, print_indent, print_open, Print};
use std::fmt;

impl Print for MJPreview {
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

impl fmt::Display for MJPreview {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.dense_print().as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn normal() {
        let item = crate::mj_preview::MJPreview::from("Hello World!");
        assert_eq!("<mj-preview>Hello World!</mj-preview>", item.dense_print());
        assert_eq!(
            "<mj-preview>\n  Hello World!\n</mj-preview>\n",
            item.pretty_print()
        );
    }
}
