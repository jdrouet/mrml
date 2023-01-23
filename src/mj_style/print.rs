#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn normal() {
        let item = crate::mj_style::MjStyle::from("Hello World!");
        assert_eq!("<mj-style>Hello World!</mj-style>", item.dense_print());
        assert_eq!("<mj-style>Hello World!</mj-style>\n", item.pretty_print());
    }
}
