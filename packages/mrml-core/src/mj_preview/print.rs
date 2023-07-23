#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn normal() {
        let item = crate::mj_preview::MjPreview::from("Hello World!");
        assert_eq!("<mj-preview>Hello World!</mj-preview>", item.dense_print());
        assert_eq!(
            "<mj-preview>Hello World!</mj-preview>\n",
            item.pretty_print()
        );
    }
}
