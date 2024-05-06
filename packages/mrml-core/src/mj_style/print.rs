use crate::prelude::print::Printable;

impl Printable for super::MjStyle {
    fn print<P: crate::prelude::print::Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.open_tag(super::NAME)?;
        printer.close_tag();
        printer.push_str(self.children.as_str());
        printer.end_tag(super::NAME)
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
    }
}
