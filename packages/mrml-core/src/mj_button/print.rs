use crate::prelude::print::{Printable, PrintableAttributes, PrintableChildren};

impl Printable for super::MjButton {
    fn print<P: crate::prelude::print::Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.open_tag(super::NAME)?;
        self.attributes.print(printer)?;
        printer.close_tag();
        self.children.print(printer)?;
        printer.end_tag(super::NAME)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let mut item = crate::mj_button::MjButton::default();
        item.attributes
            .insert("href".to_string(), "http://localhost".into());
        item.children
            .push(crate::mj_body::MjBodyChild::Text(crate::text::Text::from(
                "Hello World!".to_string(),
            )));
        assert_eq!(
            "<mj-button href=\"http://localhost\">Hello World!</mj-button>",
            item.print_dense().unwrap()
        );
    }
}
