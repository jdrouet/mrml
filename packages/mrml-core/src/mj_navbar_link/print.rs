use crate::prelude::print::{Printable, PrintableAttributes, PrintableChildren};

impl Printable for super::MjNavbarLink {
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
        let mut item = crate::mj_navbar_link::MjNavbarLink::default();
        item.attributes
            .insert("src".to_string(), "http://localhost".into());
        assert_eq!(
            "<mj-navbar-link src=\"http://localhost\" />",
            item.print_dense().unwrap()
        );
    }
}
