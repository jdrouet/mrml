use crate::prelude::print::{Printable, PrintableAttributes};

impl Printable for super::MjAttributesAll {
    fn print<P: crate::prelude::print::Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.open_tag(super::NAME)?;
        self.attributes.print(printer)?;
        printer.closed_tag();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let item = crate::mj_attributes_all::MjAttributesAll::default();
        assert_eq!("<mj-all />", item.print_dense().unwrap());
    }
}
