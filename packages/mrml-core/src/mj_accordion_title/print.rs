#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let item = crate::mj_accordion_title::MjAccordionTitle::default();
        assert_eq!("<mj-accordion-title />", item.print_dense().unwrap());
    }
}
