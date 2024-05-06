use crate::prelude::print::{PrintableAttributes, PrintableChildren, PrintableElement};

impl PrintableElement for super::MjGroup {
    fn tag(&self) -> &str {
        super::NAME
    }

    fn attributes(&self) -> &impl PrintableAttributes {
        &self.attributes
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
        let item = crate::mj_group::MjGroup::default();
        assert_eq!("<mj-group />", item.print_dense().unwrap());
    }
}
