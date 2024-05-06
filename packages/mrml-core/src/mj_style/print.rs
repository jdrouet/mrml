use crate::prelude::print::Printable;

impl Printable for super::MjStyle {
    fn print<P: crate::prelude::print::Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.push_indent();
        printer.open_tag(super::NAME)?;
        printer.close_tag();
        printer.push_new_line();
        printer.increase_indent();
        for line in self.children.split('\n') {
            printer.push_indent();
            printer.push_str(line.trim());
            printer.push_new_line();
        }
        printer.decrease_indent();
        printer.push_indent();
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
        let item = crate::mj_style::MjStyle::from("Hello World!");
        assert_eq!(
            "<mj-style>Hello World!</mj-style>",
            item.print_dense().unwrap()
        );
        assert_eq!(
            "<mj-style>\n  Hello World!\n</mj-style>\n",
            item.print_pretty().unwrap()
        );
    }
}
