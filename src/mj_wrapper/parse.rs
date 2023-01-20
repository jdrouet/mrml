#[cfg(test)]
mod tests {
    use crate::mj_wrapper::MJWrapper;
    use crate::prelude::parse::Parsable;

    #[test]
    fn parse_br_element() {
        let content = "<mj-wrapper><h1>hello</h1><br><h2>world</h2></mj-wrapper>";
        let mut tokenizer = xmlparser::Tokenizer::from(content);
        let _ = tokenizer.next().unwrap();
        let tag = xmlparser::StrSpan::from("<mj-wrapper");
        MJWrapper::parse(tag, &mut tokenizer).unwrap();
    }
}
