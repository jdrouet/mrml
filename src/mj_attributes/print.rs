#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_attributes::MJAttributes::default();
        assert_eq!("<mj-attributes />", item.dense_print());
    }
}
