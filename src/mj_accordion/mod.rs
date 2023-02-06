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

use crate::prelude::hash::Map;

pub use children::MjAccordionChild;

pub const NAME: &str = "mj-accordion";

#[derive(Debug, Default)]
#[cfg_attr(feature = "parse", derive(mrml_parse_macros::MrmlParseComponent))]
#[cfg_attr(feature = "parse", mrml_parse(child_text = false))]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintComponent))]
#[cfg_attr(feature = "print", mrml_print(tag = "NAME"))]
#[cfg_attr(feature = "json", derive(mrml_json_macros::MrmlJsonComponent))]
#[cfg_attr(feature = "json", mrml_json(tag = "NAME"))]
pub struct MjAccordion {
    pub attributes: Map<String, String>,
    pub children: Vec<MjAccordionChild>,
}

#[cfg(test)]
mod tests {
    use crate::mj_accordion::MjAccordion;
    use crate::mj_accordion_element::{MjAccordionElement, MjAccordionElementChildren};
    use crate::mj_accordion_title::MjAccordionTitle;
    use crate::prelude::parse::{is_element_start, next_token, Parsable, ParserOptions};
    use crate::prelude::print::Print;
    use crate::text::Text;
    use std::rc::Rc;
    use xmlparser::Tokenizer;

    #[test]
    fn chaining_print_parse() {
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
        let initial = element.print(false, 0, 2);
        let opts = Rc::new(ParserOptions::default());
        let mut tokenizer = Tokenizer::from("<mj-accordion><mj-accordion-element><mj-accordion-title>Hello World!</mj-accordion-title></mj-accordion-element></mj-accordion>");
        let token = next_token(&mut tokenizer).unwrap();
        let tag = is_element_start(&token).unwrap();
        let elt = MjAccordion::parse(*tag, &mut tokenizer, opts).unwrap();
        let result = elt.print(false, 0, 2);
        assert_eq!(initial, result);
    }

    #[test]
    fn chaining_json_parse() {
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
