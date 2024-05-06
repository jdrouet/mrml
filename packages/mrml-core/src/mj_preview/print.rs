use crate::prelude::print::Printable;

impl Printable for super::MjPreview {
    fn print<P: crate::prelude::print::Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.push_indent();
        printer.open_tag(super::NAME)?;
        printer.close_tag();
        printer.push_str(self.children.as_str());
        printer.end_tag(super::NAME)?;
        printer.push_new_line();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn normal() {
        let item = crate::mj_preview::MjPreview::from("Hello World!");
        assert_eq!(
            "<mj-preview>Hello World!</mj-preview>",
            item.print_dense().unwrap()
        );
    }
}
