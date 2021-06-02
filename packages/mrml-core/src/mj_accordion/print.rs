use super::{MJAccordion, MJAccordionChild, NAME};
use crate::print_attrs_children;

print_attrs_children!(MJAccordion, NAME);

impl Print for MJAccordionChild {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        match self {
            Self::Comment(elt) => elt.print(pretty, level, indent_size),
            Self::MJAccordionElement(elt) => elt.print(pretty, level, indent_size),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_accordion::MJAccordion::default();
        assert_eq!("<mj-accordion></mj-accordion>", item.dense_print());
    }
}
