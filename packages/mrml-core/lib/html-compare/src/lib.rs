#![allow(clippy::result_large_err)]

mod error;
mod helper;
mod stack;
mod token;

use std::collections::{BTreeMap, BTreeSet};

use htmlparser::{ElementEnd as HtmlElementEnd, StrSpan, Token};

use crate::error::*;
use crate::helper::cleanup_text;
use crate::token::*;

struct Cursor<'a> {
    expected: crate::stack::TokenStack<'a>,
    generated: crate::stack::TokenStack<'a>,
}

impl<'a> Cursor<'a> {
    fn new(expected_str: &'a str, generated_str: &'a str) -> Self {
        Self {
            expected: crate::stack::TokenStack::parse(expected_str).sanitize(),
            generated: crate::stack::TokenStack::parse(generated_str).sanitize(),
        }
    }

    fn next(&mut self) -> (Option<Token<'a>>, Option<Token<'a>>) {
        (self.expected.next(), self.generated.next())
    }

    fn next_attributes(
        &mut self,
    ) -> (
        (Vec<Attribute<'a>>, ElementEnd<'a>),
        (Vec<Attribute<'a>>, ElementEnd<'a>),
    ) {
        (
            Attribute::parse_all(&mut self.expected),
            Attribute::parse_all(&mut self.generated),
        )
    }
}

fn compare_attr_classes<'a>(
    expected: StrSpan<'a>,
    generated: StrSpan<'a>,
) -> Result<(), ErrorKind<'a>> {
    let exp_values = expected
        .as_str()
        .split(' ')
        .filter(|item| !item.is_empty())
        .collect::<BTreeSet<_>>();
    let res_values = generated
        .as_str()
        .split(' ')
        .filter(|item| !item.is_empty())
        .collect::<BTreeSet<_>>();

    let diff = exp_values
        .difference(&res_values)
        .copied()
        .collect::<BTreeSet<_>>();
    if !diff.is_empty() {
        return Err(ErrorKind::ExpectedClassesNotFound {
            expected,
            generated,
            difference: diff,
        });
    }

    let diff = res_values
        .difference(&exp_values)
        .copied()
        .collect::<BTreeSet<_>>();
    if !diff.is_empty() {
        return Err(ErrorKind::UnexpectedClassesFound {
            expected,
            generated,
            difference: diff,
        });
    }

    Ok(())
}

fn compare_attr_styles<'a>(
    expected: StrSpan<'a>,
    generated: StrSpan<'a>,
) -> Result<(), ErrorKind<'a>> {
    let exp_values = expected
        .as_str()
        .split(';')
        .filter(|item| !item.is_empty())
        .filter_map(|item| item.split_once(':'))
        .map(|(k, v)| (k.trim(), v.trim()))
        .collect::<BTreeMap<_, _>>();
    let gen_values = generated
        .as_str()
        .split(';')
        .filter(|item| !item.is_empty())
        .filter_map(|item| item.split_once(':'))
        .map(|(k, v)| (k.trim(), v.trim()))
        .collect::<BTreeMap<_, _>>();

    let exp_keys = exp_values.keys().cloned().collect::<BTreeSet<_>>();
    let res_keys = gen_values.keys().cloned().collect::<BTreeSet<_>>();

    let diff = exp_keys
        .difference(&res_keys)
        .copied()
        .collect::<BTreeSet<_>>();
    if !diff.is_empty() {
        return Err(ErrorKind::ExpectedStylesNotFound {
            expected,
            generated,
            difference: diff,
        });
    }

    let diff = res_keys
        .difference(&exp_keys)
        .copied()
        .collect::<BTreeSet<_>>();
    if !diff.is_empty() {
        return Err(ErrorKind::UnexpectedStylesFound {
            expected,
            generated,
            difference: diff,
        });
    }

    for (key, exp_value) in exp_values.iter() {
        if let Some(res_value) = gen_values.get(key) {
            if exp_value != res_value {
                return Err(ErrorKind::InvalidStyleValue {
                    expected,
                    generated,
                    key,
                    expected_value: exp_value,
                    generated_value: res_value,
                });
            }
        } else {
            return Err(ErrorKind::ExpectedStyleNotFound {
                expected,
                generated,
                missing: key,
            });
        }
    }

    Ok(())
}

fn compare_attributes<'a>(
    cursor: &mut Cursor<'a>,
    expected: ElementStart<'a>,
    generated: ElementStart<'a>,
) -> Result<ElementEnd<'a>, ErrorKind<'a>> {
    let ((exp_attrs, exp_end), (gen_attrs, gen_end)) = cursor.next_attributes();

    if !exp_end.end.eq(&gen_end.end) {
        return Err(ErrorKind::EndOfElementMismatch {
            expected: exp_end,
            generated: gen_end,
        });
    }

    let exp_keys = exp_attrs
        .iter()
        .filter(|attr| {
            !(["style", "class"].contains(&attr.local.as_str()) && attr.value.as_str().is_empty())
        })
        .map(|attr| (attr.local.as_str(), attr.local))
        .collect::<BTreeMap<_, _>>();
    let gen_keys = gen_attrs
        .iter()
        .map(|attr| (attr.local.as_str(), attr.local))
        .collect::<BTreeMap<_, _>>();

    let exp_str_keys = exp_keys.keys().collect::<BTreeSet<_>>();
    let gen_str_keys = gen_keys.keys().collect::<BTreeSet<_>>();

    let diff = exp_str_keys
        .difference(&gen_str_keys)
        .filter_map(|key| exp_keys.get(*key).copied())
        .collect::<Vec<_>>();
    if !diff.is_empty() {
        return Err(ErrorKind::ExpectedAttributesNotFound {
            expected,
            generated,
            expected_attributes: exp_attrs,
            generated_attributes: gen_attrs,
            difference: diff,
        });
    }
    let diff = gen_str_keys
        .difference(&exp_str_keys)
        .filter_map(|key| exp_keys.get(*key).copied())
        .collect::<Vec<_>>();
    if !diff.is_empty() {
        return Err(ErrorKind::UnexpectedAttributesFound(diff));
    }

    let exp_attrs_map = exp_attrs
        .iter()
        .map(|attr| (attr.local.as_str(), attr))
        .collect::<BTreeMap<_, _>>();
    let gen_attrs_map = gen_attrs
        .iter()
        .map(|attr| (attr.local.as_str(), attr))
        .collect::<BTreeMap<_, _>>();

    for (exp_attr, gen_attr) in exp_attrs_map.iter().filter_map(|(key, exp_value)| {
        gen_attrs_map
            .get(key)
            .map(|gen_value| (*exp_value, *gen_value))
    }) {
        if exp_attr.local == "style" {
            compare_attr_styles(exp_attr.value, gen_attr.value)?;
        } else if exp_attr.local == "class" {
            compare_attr_classes(exp_attr.value, gen_attr.value)?;
        } else if exp_attr.value.as_str() != gen_attr.value.as_str() {
            return Err(ErrorKind::InvalidAttributeValue {
                expected: exp_attr.clone(),
                generated: exp_attr.clone(),
            });
        }
    }

    Ok(exp_end)
}

fn compare_elements<'a>(
    cursor: &mut Cursor<'a>,
    expected: ElementStart<'a>,
    generated: ElementStart<'a>,
) -> Result<(), ErrorKind<'a>> {
    if !expected.local.as_str().eq(generated.local.as_str()) {
        return Err(ErrorKind::InvalidElementTag {
            expected,
            generated,
        });
    }

    let ending = compare_attributes(cursor, expected.clone(), generated.clone())?;

    if matches!(ending.end, HtmlElementEnd::Open)
        && !matches!(expected.local.as_str(), "br" | "meta")
    {
        compare_all(cursor, &expected.local, expected.span, generated.span)?;
    }

    Ok(())
}

fn compare_text<'a>(expected: StrSpan<'a>, generated: StrSpan<'a>) -> Result<(), ErrorKind<'a>> {
    if cleanup_text(&expected) != cleanup_text(&generated) {
        Err(ErrorKind::TextMismatch {
            expected,
            generated,
        })
    } else {
        Ok(())
    }
}

fn compare_comment<'a>(expected: StrSpan<'a>, result: StrSpan<'a>) -> Result<(), ErrorKind<'a>> {
    compare_text(expected, result)
}

fn compare_tokens<'a>(
    cursor: &mut Cursor<'a>,
    parent: &str,
    expected: Token<'a>,
    generated: Token<'a>,
) -> Result<(), ErrorKind<'a>> {
    match (expected, generated) {
        (Token::Comment { text: exp_text, .. }, Token::Comment { text: res_text, .. }) => {
            compare_comment(exp_text, res_text)?;
        }
        (Token::Text { text: exp_text }, Token::Text { text: res_text }) => {
            if parent == "style" {
                css_compare::compare(exp_text.as_str(), res_text.as_str()).map_err(|error| {
                    ErrorKind::CssMismatch {
                        expected: exp_text,
                        generated: res_text,
                        error,
                    }
                })?;
            } else {
                compare_text(exp_text, res_text)?;
            }
        }
        (
            Token::ElementStart {
                span: exp_span,
                prefix: exp_prefix,
                local: exp_local,
            },
            Token::ElementStart {
                span: gen_span,
                prefix: gen_prefix,
                local: gen_local,
            },
        ) => {
            compare_elements(
                cursor,
                ElementStart {
                    span: exp_span,
                    prefix: exp_prefix,
                    local: exp_local,
                },
                ElementStart {
                    span: gen_span,
                    prefix: gen_prefix,
                    local: gen_local,
                },
            )?;
        }
        (Token::ElementEnd { .. }, Token::ElementEnd { .. }) => {
            // END OF ELEMENT
            return Ok(());
        }
        (Token::ConditionalCommentStart { .. }, Token::ConditionalCommentStart { .. }) => {
            if expected.span().as_str() != generated.span().as_str() {
                return Err(ErrorKind::ElementMismatch {
                    expected,
                    generated,
                });
            }
        }
        (Token::ConditionalCommentEnd { .. }, Token::ConditionalCommentEnd { .. }) => {
            if expected.span().as_str() != generated.span().as_str() {
                return Err(ErrorKind::ElementMismatch {
                    expected,
                    generated,
                });
            }
        }
        (Token::EmptyDtd { name: exp_name, .. }, Token::EmptyDtd { name: gen_name, .. }) => {
            if exp_name.as_str() != gen_name.as_str() {
                return Err(ErrorKind::ElementMismatch {
                    expected,
                    generated,
                });
            }
        }
        (exp, gen) => {
            return Err(ErrorKind::ElementMismatch {
                expected: exp,
                generated: gen,
            });
        }
    }
    Ok(())
}

fn compare_next<'a>(
    cursor: &mut Cursor<'a>,
    parent: &str,
    expected_parent: StrSpan<'a>,
    generated_parent: StrSpan<'a>,
) -> Result<bool, ErrorKind<'a>> {
    match cursor.next() {
        (Some(expected), Some(generated)) => {
            compare_tokens(cursor, parent, expected, generated)?;
            Ok(true)
        }
        (None, None) => {
            // nothing to do
            Ok(false)
        }
        (Some(expected_element), None) => Err(ErrorKind::ExpectedElementNotFound {
            expected_parent,
            generated_parent,
            expected_element,
        }),
        (None, Some(generated)) => Err(ErrorKind::UnexpectedElementFound { generated }),
    }
}

fn compare_all<'a>(
    cursor: &mut Cursor<'a>,
    parent: &str,
    expected_parent: StrSpan<'a>,
    generated_parent: StrSpan<'a>,
) -> Result<(), ErrorKind<'a>> {
    loop {
        match compare_next(cursor, parent, expected_parent, generated_parent) {
            Ok(true) => {}
            Ok(false) => return Ok(()),
            Err(kind) => return Err(kind),
        }
    }
}

/// Compare html values without being too extreme
pub fn compare<'a>(expected: &'a str, generated: &'a str) -> Result<(), Error<'a>> {
    let mut cursor = Cursor::new(expected, generated);
    if let Err(kind) = compare_all(
        &mut cursor,
        "",
        StrSpan::from(expected),
        StrSpan::from(generated),
    ) {
        Err(Error::<'a> {
            expected,
            generated,
            kind,
        })
    } else {
        Ok(())
    }
}

pub fn assert_similar(expected: &str, generated: &str) {
    if let Err(error) = compare(expected, generated) {
        panic!("{error}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_equal() {
        assert_similar(
            include_str!("../resources/expected.html"),
            include_str!("../resources/generated.html"),
        );
    }

    #[test]
    fn simple_same_classes() {
        compare_attr_classes("foo bar baz".into(), "baz foo bar".into()).unwrap();
    }

    #[test]
    fn expected_class_not_found() {
        let err = compare_attr_classes("foo bar baz".into(), "baz bar".into()).unwrap_err();
        assert_eq!(err.display(), "ExpectedClassesNotFound { expected: StrSpan(\"foo bar baz\" 0..11), generated: StrSpan(\"baz bar\" 0..7), difference: {\"foo\"} }");
    }

    #[test]
    fn simple_same_styles() {
        compare_attr_styles(
            "width:100%;height:12px".into(),
            "width: 100%; height: 12px;".into(),
        )
        .unwrap();
    }

    #[test]
    fn expected_style_not_found() {
        let err =
            compare_attr_styles("width:100%;height:12px".into(), "width:100%".into()).unwrap_err();
        assert_eq!(err.display(), "ExpectedStylesNotFound { expected: StrSpan(\"width:100%;height:12px\" 0..22), generated: StrSpan(\"width:100%\" 0..10), difference: {\"height\"} }");
    }

    #[test]
    fn simple_same_dom() {
        compare(
            r#"<!doctype html>
<html xmlns="http://www.w3.org/1999/xhtml" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:o="urn:schemas-microsoft-com:office:office"></html>
"#,
            r#"<!doctype html>
<html xmlns="http://www.w3.org/1999/xhtml" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:o="urn:schemas-microsoft-com:office:office"></html>
"#,
        ).unwrap();
        compare(
            r#"<!doctype html>
<html xmlns="http://www.w3.org/1999/xhtml"
    xmlns:v="urn:schemas-microsoft-com:vml" xmlns:o="urn:schemas-microsoft-com:office:office">
</html>
"#,
            r#"<!doctype html>
<html  xmlns="http://www.w3.org/1999/xhtml" xmlns:o="urn:schemas-microsoft-com:office:office"  xmlns:v="urn:schemas-microsoft-com:vml">
</html>"#,
        ).unwrap();
    }

    #[test]
    fn with_conditional_dom() {
        compare(
            r#"<!doctype html>
<html xmlns="http://www.w3.org/1999/xhtml" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:o="urn:schemas-microsoft-com:office:office">
    <head>
        <!--[if !mso]><!-->
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <!--<![endif]-->
    </head>
</html>
"#,
            r#"<!doctype html>
<html xmlns="http://www.w3.org/1999/xhtml" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:o="urn:schemas-microsoft-com:office:office">
    <head>
        <!--[if !mso]><!-->
        <meta content="IE=edge" http-equiv="X-UA-Compatible">
        <!--<![endif]-->
    </head>
</html>
"#,
        ).unwrap();
    }

    #[test]
    fn with_head_style() {
        let _ = compare(r#"<!doctype html>
<html xmlns="http://www.w3.org/1999/xhtml" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:o="urn:schemas-microsoft-com:office:office">
    <head>
        <title></title>
        <!--[if !mso]><!-->
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <!--<![endif]-->
        <meta http-equiv="Content-Type" content="text/html; charset=UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <style type="text/css">
            #outlook a {
                padding: 0;
            }

            body {
                margin: 0;
                padding: 0;
                -webkit-text-size-adjust: 100%;
                -ms-text-size-adjust: 100%;
            }
        </style>
    </head>
</html>"#, r#"<!doctype html>
<html xmlns="http://www.w3.org/1999/xhtml" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:o="urn:schemas-microsoft-com:office:office">
    <head>
        <title></title>
        <!--[if !mso]><!-->
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <!--<![endif]-->
        <meta http-equiv="Content-Type" content="text/html; charset=UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <style type="text/css">
            body {
                margin: 0;
                padding: 0;
                -webkit-text-size-adjust: 100%;
                -ms-text-size-adjust: 100%;
            }
        </style>
    </head>
</html>"#).unwrap_err();
    }

    #[test]
    fn with_empty_head_style() {
        compare(
            r#"<!doctype html>
<html>
    <head>
        <title></title>
        <style type="text/css">
        </style>
    </head>
</html>"#,
            r#"<!doctype html>
<html>
    <head>
        <title></title>
    </head>
</html>"#,
        )
        .expect("should be equal");
    }
}
