use crate::prelude::print::{Printable, PrintableAttributes};

impl Printable for super::MjHtmlAttribute {
    fn print<P: crate::prelude::print::Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.push_indent();
        printer.open_tag(super::NAME)?;
        printer.push_attribute("name", self.attributes.name.as_str())?;
        printer.close_tag();
        printer.push_str(self.children.as_str());
        printer.end_tag(super::NAME)?;
        printer.push_new_line();
        Ok(())
    }
}

impl PrintableAttributes for super::MjHtmlAttributeAttributes {
    fn print<P: crate::prelude::print::Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.push_attribute("name", self.name.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let item = crate::mj_html_attribute::MjHtmlAttribute::default();
        assert_eq!(
            r#"<mj-html-attribute name=""></mj-html-attribute>"#,
            item.print_dense().unwrap()
        )
    }
}
