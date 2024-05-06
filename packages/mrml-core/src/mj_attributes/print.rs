use crate::prelude::print::{PrintableChildren, PrintableElement};

impl PrintableElement for super::MjAttributes {
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
        let item = crate::mj_attributes::MjAttributes::default();
        assert_eq!("<mj-attributes />", item.print_dense().unwrap());
    }
}
