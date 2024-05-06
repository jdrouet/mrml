use crate::prelude::print::{Printable, PrintableAttributes};

impl Printable for super::MjAttributesClass {
    fn print<P: crate::prelude::print::Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.push_indent();
        printer.open_tag(super::NAME)?;
        printer.push_attribute("name", self.name.as_str())?;
        self.attributes.print(printer)?;
        printer.closed_tag();
        printer.push_new_line();
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
