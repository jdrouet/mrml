use crate::prelude::print::{Printable, PrintableAttributes};

impl Printable for super::MjAttributesElement {
    fn print<P: crate::prelude::print::Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.open_tag(self.name.as_str())?;
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
        let item = crate::mj_attributes_element::MjAttributesElement::new("span".to_string());
        assert_eq!("<span />", item.print_dense().unwrap());
    }
}
