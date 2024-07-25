#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let item = crate::mj_attributes::MjAttributes::default();
        assert_eq!("<mj-attributes />", item.print_dense().unwrap());
    }
}
