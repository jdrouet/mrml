#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let item = crate::mj_html_attributes::MjHtmlAttributes::default();
        assert_eq!("<mj-html-attributes />", item.print_dense().unwrap());
    }
}
