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
