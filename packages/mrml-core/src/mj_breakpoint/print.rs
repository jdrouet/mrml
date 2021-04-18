use super::MJBreakpoint;
use crate::prelude::print::{self, Print};
use crate::print_display;
use std::collections::HashMap;

impl Print for MJBreakpoint {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        let mut attrs = HashMap::<String, String>::new();
        attrs.insert("width".to_string(), self.attributes.width.clone());
        print::open(super::NAME, Some(&attrs), true, pretty, level, indent_size)
    }
}

print_display!(MJBreakpoint);

#[cfg(test)]
mod tests {
    use crate::mj_breakpoint::{MJBreakpoint, MJBreakpointAttributes};
    use crate::prelude::print::Print;

    #[test]
    fn normal() {
        let item = MJBreakpoint {
            attributes: MJBreakpointAttributes {
                width: String::from("10px"),
            },
        };
        assert_eq!("<mj-breakpoint width=\"10px\" />", item.dense_print());
    }
}
