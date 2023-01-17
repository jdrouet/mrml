#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_column::MJColumn::default();
        assert_eq!("<mj-column />", item.dense_print());
    }
}
