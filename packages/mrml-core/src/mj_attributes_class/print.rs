use crate::prelude::print::PrintableAttributes;

impl PrintableAttributes for super::MjAttributesClassAttributes {
    fn print<P: crate::prelude::print::Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.push_attribute("name", self.name.as_str())?;
        self.others.print(printer)
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
