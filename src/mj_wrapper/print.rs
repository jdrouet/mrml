#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_wrapper::MJWrapper::default();
        assert_eq!("<mj-wrapper />", item.dense_print());
    }
}
