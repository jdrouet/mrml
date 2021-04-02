use super::MJBreakpoint;
use crate::prelude::print::{print_open, Print};
use std::collections::HashMap;
use std::fmt;

impl Print for MJBreakpoint {
    fn print(&self, f: &mut String, pretty: bool, level: usize, indent_size: usize) {
        let mut attrs = HashMap::<String, String>::new();
        attrs.insert("value".to_string(), self.value.clone());
        print_open(
            f,
            super::NAME,
            Some(&attrs),
            true,
            pretty,
            level,
            indent_size,
        );
        if pretty {
            f.push('\n');
        }
    }
}

impl fmt::Display for MJBreakpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.dense_print().as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn normal() {
        let item = crate::mj_breakpoint::MJBreakpoint {
            value: String::from("10px"),
        };
        assert_eq!("<mj-breakpoint value=\"10px\" />", item.dense_print());
    }
}
