use super::Comment;
use crate::prelude::print::{print_indent, Print};
use std::fmt;

impl Print for Comment {
    fn print(&self, f: &mut String, pretty: bool, level: usize, indent_size: usize) {
        if pretty {
            print_indent(f, level, indent_size);
        }
        f.push_str("<!--");
        f.push_str(self.0.as_str());
        f.push_str("-->");
        if pretty {
            f.push_str("\n");
        }
    }
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.dense_print().as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::comment::Comment::from("Hello World");
        assert_eq!("<!--Hello World-->", item.dense_print());
    }
}
