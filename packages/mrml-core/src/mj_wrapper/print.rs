#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let item = crate::mj_wrapper::MjWrapper::default();
        assert_eq!("<mj-wrapper />", item.print_dense().unwrap());
    }
}
