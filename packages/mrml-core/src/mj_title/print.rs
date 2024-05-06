use crate::prelude::print::Printable;

impl Printable for super::MjTitle {
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
        let item = crate::mj_title::MjTitle::from("Hello World!");
        assert_eq!(
            "<mj-title>Hello World!</mj-title>",
            item.print_dense().unwrap()
        );
        assert_eq!(
            "<mj-title>Hello World!</mj-title>\n",
            item.print_pretty().unwrap()
        );
    }
}
