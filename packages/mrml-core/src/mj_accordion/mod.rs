//! Module containing the `mj-accordion` element as defined in [the documentation](https://documentation.mjml.io/#mj-accordion).
//!
//! ```rust
//! let template = include_str!("../../resources/compare/success/mj-accordion.mjml");
//! let root = mrml::parse(template).expect("parse template");
//! let opts = mrml::prelude::render::Options::default();
//! match root.render(&opts) {
//!     Ok(content) => println!("{content}"),
//!     Err(_) => println!("couldn't render mjml template"),
//! };
//! ```

mod children;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub use children::MjAccordionChild;

use crate::prelude::hash::Map;

pub const NAME: &str = "mj-accordion";

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(mrml_json_macros::MrmlJsonComponent))]
#[cfg_attr(feature = "json", mrml_json(tag = "NAME"))]
pub struct MjAccordion {
    pub attributes: Map<String, String>,
    pub children: Vec<MjAccordionChild>,
}

#[cfg(all(test, feature = "parse", feature = "render"))]
mod tests {
    #[cfg(feature = "print")]
    #[test]
    fn chaining_print_parse() {
        use crate::mj_accordion::MjAccordion;
        use crate::mj_accordion_element::{MjAccordionElement, MjAccordionElementChildren};
        use crate::mj_accordion_title::MjAccordionTitle;
        use crate::prelude::parser::{MrmlCursor, MrmlParser, ParserOptions};
        use crate::prelude::print::Printable;
        use crate::text::Text;

        let element = MjAccordion {
            attributes: Default::default(),
            children: vec![MjAccordionElement {
                attributes: Default::default(),
                children: MjAccordionElementChildren {
                    title: Some(MjAccordionTitle {
                        attributes: Default::default(),
                        children: vec![Text::from("Hello World!".to_string())],
                    }),
                    text: None,
                },
            }
            .into()],
        };
        let initial = element.print_dense().unwrap();
        let raw ="<mj-accordion><mj-accordion-element><mj-accordion-title>Hello World!</mj-accordion-title></mj-accordion-element></mj-accordion>";
        let opts = ParserOptions::default();
        let parser = MrmlParser::new(&opts);
        let mut cursor = MrmlCursor::new(raw);
        let elt: MjAccordion = parser.parse_root(&mut cursor).unwrap();
        let result = elt.print_dense().unwrap();
        assert_eq!(initial, result);
    }

    #[cfg(feature = "json")]
    #[test]
    fn chaining_json_parse() {
        use crate::mj_accordion::MjAccordion;
        use crate::mj_accordion_element::{MjAccordionElement, MjAccordionElementChildren};
        use crate::mj_accordion_title::MjAccordionTitle;
        use crate::text::Text;

        let element = MjAccordion {
            attributes: Default::default(),
            children: vec![MjAccordionElement {
                attributes: Default::default(),
                children: MjAccordionElementChildren {
                    title: Some(MjAccordionTitle {
                        attributes: Default::default(),
                        children: vec![Text::from("Hello World!".to_string())],
                    }),
                    text: None,
                },
            }
            .into()],
        };
        let initial_json = serde_json::to_string(&element).unwrap();
        let result: MjAccordion = serde_json::from_str(initial_json.as_str()).unwrap();

        let result_json = serde_json::to_string(&result).unwrap();
        assert_eq!(initial_json, result_json);
    }
}
