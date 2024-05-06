use crate::prelude::print::{PrintableChildren, PrintableElement};

impl PrintableElement for super::MjRaw {
    fn tag(&self) -> &str {
        super::NAME
    }

    fn children(&self) -> &impl PrintableChildren {
        &self.children
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let item = crate::mj_raw::MjRaw::default();
        assert_eq!("<mj-raw />", item.print_dense().unwrap());
    }
}
