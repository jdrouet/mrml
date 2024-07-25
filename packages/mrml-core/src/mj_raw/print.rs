#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let item = crate::mj_raw::MjRaw::default();
        assert_eq!("<mj-raw />", item.print_dense().unwrap());
    }
}
