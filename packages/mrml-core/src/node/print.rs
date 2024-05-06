use super::Node;
use crate::prelude::{
    is_void_element,
    print::{Printable, PrintableAttributes, PrintableChildren, Printer},
};

impl<T: Printable> Printable for Node<T> {
    fn print<P: Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.push_indent();
        let tag = self.tag.as_str();
        printer.open_tag(tag)?;
        self.attributes.print(printer)?;
        if self.children.is_empty() && (tag.starts_with("mj") || is_void_element(tag)) {
            printer.closed_tag();
            printer.push_new_line();
        } else {
            printer.close_tag();
            printer.push_new_line();
            printer.increase_indent();
            self.children.print(printer)?;
            printer.decrease_indent();
            printer.push_indent();
            printer.end_tag(tag)?;
            printer.push_new_line();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_body::MjBodyChild;
    use crate::mj_raw::MjRawChild;
    use crate::prelude::print::Printable;

    #[test]
    fn normal() {
        let item = crate::node::Node::<MjBodyChild>::from("span");
        assert_eq!("<span></span>", item.print_dense().unwrap());
        let item = crate::node::Node::<MjRawChild>::from("span");
        assert_eq!("<span></span>", item.print_dense().unwrap());
    }

    #[test]
    fn with_attributes() {
        let mut item = crate::node::Node::<MjBodyChild>::from("span");
        item.attributes
            .insert("color".to_string(), "red".to_string());
        item.children
            .push(crate::node::Node::from("b".to_string()).into());
        assert_eq!(
            "<span color=\"red\"><b></b></span>",
            item.print_dense().unwrap()
        );
    }
}
