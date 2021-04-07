use super::MJPreview;
use crate::prelude::print::{self, Print};
use std::fmt;

impl Print for MJPreview {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        if pretty {
            print::indent(level, indent_size, self.print(false, level, indent_size))
        } else {
            format!("<{}>{}</{}>", super::NAME, self.0, super::NAME)
        }
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
            "<mj-preview>Hello World!</mj-preview>\n",
            item.pretty_print()
        );
    }
}
