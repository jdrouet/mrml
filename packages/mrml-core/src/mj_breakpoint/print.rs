#[cfg(test)]
mod tests {
    use crate::mj_breakpoint::{MjBreakpoint, MjBreakpointAttributes};
    use crate::prelude::print::Print;

    #[test]
    fn normal() {
        let item = MjBreakpoint {
            attributes: MjBreakpointAttributes {
                width: String::from("10px"),
            },
        };
        assert_eq!("<mj-breakpoint width=\"10px\" />", item.dense_print());
    }
}
