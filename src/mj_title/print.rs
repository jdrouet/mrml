#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn normal() {
        let item = crate::mj_title::MjTitle::from("Hello World!");
        assert_eq!("<mj-title>Hello World!</mj-title>", item.dense_print());
        assert_eq!("<mj-title>Hello World!</mj-title>\n", item.pretty_print());
    }
}
