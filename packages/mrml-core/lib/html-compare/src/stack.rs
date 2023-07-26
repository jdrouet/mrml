use std::collections::VecDeque;

use htmlparser::{Token, Tokenizer};

fn is_head_style_element_start(token: Token<'_>) -> bool {
    if let Token::ElementStart { prefix, local, .. } = token {
        prefix.as_str().is_empty() && local.as_str() == "style"
    } else {
        false
    }
}

fn take_until_element_end<'a>(items: &mut VecDeque<Token<'a>>) -> (Vec<Token<'a>>, Token<'a>) {
    let mut buffer = Vec::new();
    while let Some(token) = items.pop_front() {
        match token {
            Token::Attribute { .. } => buffer.push(token),
            Token::ElementEnd { .. } => return (buffer, token),
            other => panic!("unexpected element {other:?}"),
        }
    }
    panic!("unexpected end")
}

pub struct TokenStack<'a> {
    pub inner: VecDeque<Token<'a>>,
}

impl<'a> TokenStack<'a> {
    pub fn parse(input: &'a str) -> Self {
        let mut inner = VecDeque::new();
        let mut tokenizer = Tokenizer::from(input);
        while let Some(Ok(token)) = tokenizer.next() {
            inner.push_back(token);
        }
        Self { inner }
    }

    fn sanitize_text(self) -> Self {
        Self {
            inner: self
                .inner
                .into_iter()
                .filter(|item| match item {
                    // removes empty comments
                    Token::Comment { text, .. } => {
                        !crate::helper::cleanup_text(text.as_str()).is_empty()
                    }
                    // removes empty text
                    Token::Text { text } => !crate::helper::cleanup_text(text.as_str()).is_empty(),
                    _ => true,
                })
                .collect(),
        }
    }

    fn sanitize_empty_head_style(mut self) -> Self {
        let mut inner = VecDeque::with_capacity(self.inner.len());
        while let Some(token) = self.inner.pop_front() {
            if is_head_style_element_start(token) {
                let (buf, tail) = take_until_element_end(&mut self.inner);
                if matches!(
                    tail,
                    Token::ElementEnd {
                        end: htmlparser::ElementEnd::Empty,
                        ..
                    }
                ) {
                    continue;
                } else if matches!(
                    tail,
                    Token::ElementEnd {
                        end: htmlparser::ElementEnd::Open,
                        ..
                    }
                ) && matches!(
                    self.inner.front(),
                    Some(Token::ElementEnd {
                        end: htmlparser::ElementEnd::Close(_, _),
                        ..
                    })
                ) {
                    let _ = self.inner.pop_front();
                    continue;
                } else {
                    inner.push_back(token);
                    for item in buf {
                        inner.push_back(item);
                    }
                    inner.push_back(tail);
                }
            } else {
                inner.push_back(token);
            }
        }
        Self { inner }
    }

    fn sanitize_close_and_open_condition(mut self) -> Self {
        let mut inner = VecDeque::with_capacity(self.inner.len());
        let mut previous_condition = None;
        while let Some(token) = self.inner.pop_front() {
            match token {
                Token::ConditionalCommentStart { condition, .. } => {
                    previous_condition = Some(condition.as_str());
                    inner.push_back(token);
                }
                Token::ConditionalCommentEnd { .. } => {
                    match self.inner.pop_front() {
                        Some(Token::ConditionalCommentStart { condition, .. })
                            if Some(condition.as_str()) == previous_condition =>
                        {
                            // do nothing
                        }
                        Some(next) => {
                            self.inner.push_front(next);
                            inner.push_back(token);
                        }
                        None => {
                            inner.push_back(token);
                        }
                    }
                    previous_condition = None;
                }
                other => {
                    inner.push_back(other);
                }
            }
        }
        Self { inner }
    }

    pub fn sanitize(self) -> Self {
        self.sanitize_text()
            .sanitize_close_and_open_condition()
            .sanitize_empty_head_style()
    }

    pub fn next(&mut self) -> Option<Token<'a>> {
        self.inner.pop_front()
    }

    pub fn head(&mut self) -> Option<Token<'a>> {
        self.inner.front().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize() {
        let stack = TokenStack::parse(r#"<!doctype html>
<html xmlns="http://www.w3.org/1999/xhtml" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:o="urn:schemas-microsoft-com:office:office">

    <head>
    <title></title>
    <!--[if !mso]><!-->
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <!--<![endif]-->
    <style type="text/css">
        p {
        display: block;
        margin: 13px 0;
        }
    </style>
    <!--[if mso]>
<noscript>
<xml>
<o:OfficeDocumentSettings>
<o:AllowPNG/>
<o:PixelsPerInch>96</o:PixelsPerInch>
</o:OfficeDocumentSettings>
</xml>
</noscript>
<![endif]-->
    <!--[if lte mso 11]>
<style type="text/css">
.mj-outlook-group-fix { width:100% !important; }
</style>
<![endif]-->
    <style type="text/css">
        @media only screen and (min-width:480px) {
        .mj-column-per-100 {
            width: 100% !important;
            max-width: 100%;
        }
        }
    </style>"#).sanitize();
        let result = stack
            .inner
            .iter()
            .filter(|t| {
                matches!(
                    t,
                    Token::ConditionalCommentStart { .. }
                        | Token::ConditionalCommentEnd { .. }
                        | Token::ElementStart { .. }
                        | Token::ElementEnd { .. }
                )
            })
            .map(|t| t.span().as_str())
            .collect::<Vec<_>>();
        similar_asserts::assert_eq!(
            result,
            vec![
                "<html",
                ">",
                "<head",
                ">",
                "<title",
                ">",
                "</title>",
                "<!--[if !mso]><!-->",
                "<meta",
                ">",
                "<!--<![endif]-->",
                "<style",
                ">",
                "</style>",
                "<!--[if mso]>",
                "<noscript",
                ">",
                "<xml",
                ">",
                "<o:OfficeDocumentSettings",
                ">",
                "<o:AllowPNG",
                "/>",
                "<o:PixelsPerInch",
                ">",
                "</o:PixelsPerInch>",
                "</o:OfficeDocumentSettings>",
                "</xml>",
                "</noscript>",
                "<![endif]-->",
                "<!--[if lte mso 11]>",
                "<style",
                ">",
                "</style>",
                "<![endif]-->",
                "<style",
                ">",
                "</style>"
            ]
        );
    }

    #[test]
    fn remove_duplication_conditions() {
        let expected =
            TokenStack::parse("<br><!--[if mso | IE]></td> <![endif]--> <!--[if mso | IE]></tr>")
                .sanitize();
        let result = TokenStack::parse("<br><!--[if mso | IE]></td></tr>");
        assert_eq!(
            expected
                .inner
                .into_iter()
                .map(|token| token.span().as_str())
                .collect::<Vec<_>>(),
            result
                .sanitize()
                .inner
                .into_iter()
                .map(|token| token.span().as_str())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn keep_different_conditions() {
        let expected = TokenStack::parse(
            "<first><!--[if mso]><second><![endif]--><!--[if lte mso 11]></third><![endif]-->",
        )
        .sanitize();
        let result = TokenStack::parse(
            "<first><!--[if mso]><second><![endif]--><!--[if lte mso 11]></third><![endif]-->",
        );
        assert_eq!(
            expected
                .inner
                .into_iter()
                .map(|token| token.span().as_str())
                .collect::<Vec<_>>(),
            result
                .inner
                .into_iter()
                .map(|token| token.span().as_str())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn remove_head_style() {
        let expected = TokenStack::parse("<head></head>");
        let result = TokenStack::parse("<head><style type=\"text/css\"></style></head>");
        assert_eq!(
            expected
                .inner
                .into_iter()
                .map(|token| token.span().as_str())
                .collect::<Vec<_>>(),
            result
                .sanitize()
                .inner
                .into_iter()
                .map(|token| token.span().as_str())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn keep_head_style() {
        let expected = TokenStack::parse("<head><style type=\"text/css\">foo {}</style></head>");
        let result = TokenStack::parse("<head><style type=\"text/css\">foo {}</style></head>");
        assert_eq!(
            expected
                .inner
                .into_iter()
                .map(|token| token.span().as_str())
                .collect::<Vec<_>>(),
            result
                .sanitize()
                .inner
                .into_iter()
                .map(|token| token.span().as_str())
                .collect::<Vec<_>>()
        );
    }
}
