use super::MJFont;
use crate::prelude::print::{print_indent, Print};
use std::fmt;

impl Print for MJFont {
    fn print(&self, f: &mut String, pretty: bool, level: usize, indent_size: usize) {
        if pretty {
            print_indent(f, level, indent_size);
        }
        f.push_str("<mj-font name=\"");
        f.push_str(&self.name);
        f.push_str("\" href=\"");
        f.push_str(&self.href);
        f.push_str("\" />");
        if pretty {
            f.push_str("\n");
        }
    }
}

impl fmt::Display for MJFont {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.dense_print().as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn normal() {
        let item = crate::mj_font::MJFont {
            name: String::from("Comic sans MS"),
            href: String::from("http://localhost"),
        };
        assert_eq!(
            "<mj-font name=\"Comic sans MS\" href=\"http://localhost\" />",
            item.dense_print()
        );
    }
}
