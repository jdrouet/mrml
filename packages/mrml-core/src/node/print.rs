use super::Node;
use crate::prelude::print::{print_close, print_indent, print_open, Print};
use std::fmt;

impl<T: Print> Print for Node<T> {
    fn print(&self, f: &mut String, pretty: bool, level: usize, indent_size: usize) {
        print_open(
            f,
            &self.tag,
            Some(&self.attributes),
            false,
            pretty,
            level,
            indent_size,
        );
        if pretty {
            print_indent(f, level + 1, indent_size);
        }
        self.children.iter().for_each(|child| {
            child.print(f, pretty, level + 1, indent_size);
        });
        print_close(f, &self.tag, pretty, level, indent_size);
    }
}

impl<T: Print> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.dense_print().as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_body::MJBodyChild;
    use crate::mj_raw::MJRawChild;
    use crate::prelude::print::Print;

    #[test]
    fn normal() {
        let item = crate::node::Node::<MJBodyChild>::from("span");
        assert_eq!("<span></span>", item.dense_print());
        let item = crate::node::Node::<MJRawChild>::from("span");
        assert_eq!("<span></span>", item.dense_print());
    }

    #[test]
    fn with_attributes() {
        let mut item = crate::node::Node::<MJBodyChild>::from("span");
        item.attributes
            .insert("color".to_string(), "red".to_string());
        item.children
            .push(crate::node::Node::from("b".to_string()).into());
        assert_eq!("<span color=\"red\"><b></b></span>", item.dense_print());
    }
}
