use crate::prelude::print::{Printable, PrintableChildren};

impl Printable for super::MjAccordionTitle {
    fn print<P: crate::prelude::print::Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.open_tag(super::NAME)?;
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
        let item = crate::mj_accordion_title::MjAccordionTitle::default();
        assert_eq!("<mj-accordion-title />", item.print_dense().unwrap());
    }
}
