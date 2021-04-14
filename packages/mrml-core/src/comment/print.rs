use super::Comment;
use crate::prelude::print::{self, Print};
use std::fmt;

impl Print for Comment {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        if pretty {
            print::indent(level, indent_size, self.print(false, level, indent_size))
        } else {
            format!("<!--{}-->", self.children)
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
