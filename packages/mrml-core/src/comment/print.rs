use super::Comment;
use crate::prelude::print::{self, Print};
use crate::print_display;

impl Print for Comment {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        if pretty {
            print::indent(level, indent_size, self.print(false, level, indent_size))
        } else {
            format!("<!--{}-->", self.children)
        }
    }
}

print_display!(Comment);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::comment::Comment::from("Hello World");
        assert_eq!("<!--Hello World-->", item.dense_print());
    }
}
