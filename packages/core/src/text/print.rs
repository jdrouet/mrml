use super::Text;
use crate::prelude::print::{print_indent, Print};
use std::fmt;

impl Print for Text {
    fn print(&self, f: &mut String, pretty: bool, level: usize, indent_size: usize) {
        if pretty {
            print_indent(f, level, indent_size);
        }
        f.push_str(&self.0);
        if pretty {
            f.push('\n');
        }
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.dense_print().as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::text::Text::from("Hello World");
        assert_eq!("Hello World", item.dense_print());
    }
}
