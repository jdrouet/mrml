use super::MJBody;
use crate::prelude::print::{print_close, print_open, Print};
use std::fmt;

impl Print for MJBody {
    fn print(&self, f: &mut String, pretty: bool, level: usize, indent_size: usize) {
        print_open(
            f,
            super::NAME,
            Some(&self.attributes),
            false,
            pretty,
            level,
            indent_size,
        );
        self.children.iter().for_each(|child| {
            child.as_print().print(f, pretty, level + 1, indent_size);
        });
        print_close(f, super::NAME, pretty, level, indent_size);
    }
}

impl fmt::Display for MJBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.dense_print().as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_body::MJBody::default();
        assert_eq!("<mj-body></mj-body>", item.dense_print());
    }

    #[test]
    fn with_children() {
        let mut item = crate::mj_body::MJBody::default();
        item.attributes
            .insert("background-color".to_string(), "red".to_string());
        item.children
            .push(crate::mj_body::MJBodyChild::from(crate::node::Node::from(
                "span",
            )));
        assert_eq!(
            "<mj-body background-color=\"red\"><span></span></mj-body>",
            item.dense_print()
        );
    }
}
