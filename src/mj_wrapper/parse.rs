#[cfg(test)]
mod tests {
    use crate::mj_wrapper::MjWrapper;
    use crate::prelude::parse::{Parsable, ParserOptions};
    use std::rc::Rc;

    #[test]
    fn parse_br_element() {
        let opts = Rc::new(ParserOptions::default());
        let content = "<mj-wrapper><h1>hello</h1><br><h2>world</h2></mj-wrapper>";
        let mut tokenizer = xmlparser::Tokenizer::from(content);
        let _ = tokenizer.next().unwrap();
        let tag = xmlparser::StrSpan::from("<mj-wrapper");
        MjWrapper::parse(tag, &mut tokenizer, opts).unwrap();
    }
}
