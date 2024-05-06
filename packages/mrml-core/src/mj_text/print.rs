use crate::prelude::print::{Printable, PrintableAttributes, PrintableChildren};

impl Printable for super::MjText {
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
        let mut item = crate::mj_text::MjText::default();
        item.attributes
            .insert("href".to_string(), "http://localhost".into());
        item.children
            .push(crate::text::Text::from(String::from("test")).into());
        assert_eq!(
            "<mj-text href=\"http://localhost\">test</mj-text>",
            item.print_dense().unwrap()
        );
    }
}
