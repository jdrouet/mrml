#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_raw::MjRaw::default();
        assert_eq!("<mj-raw />", item.dense_print());
    }
}
