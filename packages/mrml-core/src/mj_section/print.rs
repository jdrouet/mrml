#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let item = crate::mj_section::MjSection::default();
        assert_eq!("<mj-section />", item.print_dense().unwrap());
    }
}
