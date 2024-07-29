#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let item = crate::mj_column::MjColumn::default();
        assert_eq!("<mj-column />", item.print_dense().unwrap());
    }
}
