use crate::prelude::print::PrintableAttributes;

impl PrintableAttributes for super::MjSelectorAttributes {
    fn print<P: crate::prelude::print::Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.push_attribute("path", self.path.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let item = crate::mj_html_attributes::MjHtmlAttributes::default();
        assert_eq!("<mj-selector />", item.print_dense().unwrap());
    }
}
