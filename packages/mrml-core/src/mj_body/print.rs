use crate::prelude::print::{Printable, PrintableAttributes, PrintableChildren};

impl Printable for super::MjBody {
    fn print<P: crate::prelude::print::Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.open_tag(super::NAME)?;
        self.attributes.print(printer)?;
        if self.children.is_empty() {
            printer.closed_tag();
        } else {
            printer.close_tag();
            self.children.print(printer)?;
            printer.end_tag(super::NAME)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let item = crate::mj_body::MjBody::default();
        assert_eq!("<mj-body />", item.print_dense().unwrap());
    }

    #[test]
    fn with_children() {
        let mut item = crate::mj_body::MjBody::default();
        item.attributes
            .insert("background-color".to_string(), "red".to_string());
        item.children
            .push(crate::mj_body::MjBodyChild::from(crate::node::Node::from(
                "span",
            )));
        assert_eq!(
            "<mj-body background-color=\"red\"><span></span></mj-body>",
            item.print_dense().unwrap()
        );
    }
}
