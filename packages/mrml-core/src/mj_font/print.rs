#[cfg(test)]
mod tests {
    use crate::mj_font::{MjFont, MjFontAttributes};
    use crate::prelude::print::Print;

    #[test]
    fn normal() {
        let item = MjFont {
            attributes: MjFontAttributes {
                name: String::from("Comic sans MS"),
                href: String::from("http://localhost"),
            },
        };
        assert_eq!(
            "<mj-font href=\"http://localhost\" name=\"Comic sans MS\" />",
            item.dense_print()
        );
    }
}
