use crate::prelude::print::PrintableAttributes;

impl PrintableAttributes for super::MjSelectorAttributes {
    fn print<P: crate::prelude::print::Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.push_attribute("path", self.path.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_selector::{MjSelector, MjSelectorAttributes};
    use crate::prelude::print::Printable;

    #[test]
    fn normal() {
        let item = MjSelector::new(
            MjSelectorAttributes {
                path: String::from(".cool_class"),
            },
            Vec::new(),
        );

        assert_eq!(
            "<mj-selector path=\".cool_class\" />",
            item.print_dense().unwrap()
        );
    }

    #[test]
    fn empty() {
        let item = MjSelector::new(
            MjSelectorAttributes {
                path: String::from(""),
            },
            Vec::new(),
        );
        assert_eq!("<mj-selector path=\"\" />", item.print_dense().unwrap());
    }
}
