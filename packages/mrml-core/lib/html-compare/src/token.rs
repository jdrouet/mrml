use htmlparser::StrSpan;

#[derive(Clone, Debug)]
pub struct Attribute<'a> {
    pub prefix: StrSpan<'a>,
    pub local: StrSpan<'a>,
    pub value: StrSpan<'a>,
    pub span: StrSpan<'a>,
}

impl<'a> Attribute<'a> {
    pub fn parse_all(
        stack: &mut crate::stack::TokenStack<'a>,
    ) -> (Vec<Attribute<'a>>, ElementEnd<'a>) {
        let mut result = Vec::new();
        loop {
            match stack.next() {
                Some(htmlparser::Token::Attribute {
                    prefix,
                    local,
                    value,
                    span,
                }) => {
                    result.push(Attribute {
                        prefix,
                        local,
                        value,
                        span,
                    });
                }
                Some(htmlparser::Token::ElementEnd { end, span }) => {
                    return (result, ElementEnd { end, span })
                }
                _ => panic!("invalid token in attributes"),
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ElementStart<'a> {
    pub prefix: StrSpan<'a>,
    pub local: StrSpan<'a>,
    pub span: StrSpan<'a>,
}

#[derive(Debug)]
pub struct ElementEnd<'a> {
    pub end: htmlparser::ElementEnd<'a>,
    pub span: StrSpan<'a>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ElementOpen<'a> {
    pub start: ElementStart<'a>,
    pub attributes: Vec<Attribute<'a>>,
    pub end: ElementEnd<'a>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ElementClose<'a> {
    pub span: StrSpan<'a>,
}
