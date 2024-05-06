use crate::prelude::print::{PrintableAttributes, PrintableElement};

impl PrintableAttributes for super::MjBreakpointAttributes {
    fn print<P: crate::prelude::print::Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.push_attribute("width", self.width.as_str())
    }
}

impl PrintableElement for super::MjBreakpoint {
    fn tag(&self) -> &str {
        super::NAME
    }

    fn attributes(&self) -> &impl PrintableAttributes {
        &self.attributes
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_breakpoint::{MjBreakpoint, MjBreakpointAttributes};
    use crate::prelude::print::Printable;

    #[test]
    fn normal() {
        let item = MjBreakpoint {
            attributes: MjBreakpointAttributes {
                width: String::from("10px"),
            },
        };
        assert_eq!(
            "<mj-breakpoint width=\"10px\" />",
            item.print_dense().unwrap()
        );
    }
}
