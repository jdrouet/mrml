use crate::helper::condition;
use std::collections::{BTreeMap, BTreeSet, LinkedList};
use xmlparser::{ElementEnd, Token, Tokenizer};

fn trim_header_comment(input: &str) -> String {
    if input.starts_with("<!-- FILE:") {
        if let Some(index) = input.find('\n') {
            return input.split_at(index).1.to_string();
        }
    }
    input.to_string()
}

fn cleanup(input: &str) -> String {
    trim_header_comment(input)
        // conditions and comments
        .replace(condition::END_NEGATION_CONDITIONAL_TAG, " ENDIF_NEGATION ")
        .replace(condition::END_CONDITIONAL_TAG, " ENDIF ")
        .replace(condition::START_MSO_NEGATION_CONDITIONAL_TAG, " IF_NO_MSO ")
        .replace(
            condition::START_NEGATION_CONDITIONAL_TAG,
            " IF_NO_MSO_OR_IE ",
        )
        .replace("<!--[if !mso><!-->", " IF_NO_MSO ")
        .replace("<!--[if !mso]><!-- -->", " IF_NO_MSO ")
        .replace(condition::START_MSO_CONDITIONAL_TAG, " IF_MSO ")
        .replace(condition::START_IE11_CONDITIONAL_TAG, " IF_LTE_MSO_11 ")
        .replace(condition::START_CONDITIONAL_TAG, " IF_MSO_OR_IE ")
        // TODO handle removing closing and opening conditions
        .replace(" ENDIF  IF_MSO ", "")
        .replace(" ENDIF  IF_MSO_OR_IE ", "")
        // empty style header blocks
        .replace("<style type=\"text/css\">\n  </style>", "")
        // empty style attributes
        .replace("style=\"\"", "")
        // empty class attributes
        .replace("class=\"\"", "")
        // empty divs
        .replace("<div></div>", "")
        //
        .replace("<!doctype html>\n", "")
        .replace("<!doctype html>", "")
}

pub fn cleanup_text(input: &str) -> String {
    input.replace(' ', "").replace('\t', "").replace('\n', "")
}

fn compare_style<'a>(path: &mut LinkedList<&'a str>, expected: &str, result: &str) {
    let expected_set = expected
        .split(';')
        .filter(|item| !item.is_empty())
        .map(|item| item.trim().to_string())
        .collect::<Vec<String>>();
    let result_set = result
        .split(';')
        .filter(|item| !item.is_empty())
        .map(|item| item.trim().to_string())
        .collect::<Vec<String>>();
    assert_eq!(expected_set, result_set, "different styles in {:?}", path);
}

fn compare_text<'a>(path: &mut LinkedList<&'a str>, expected: &str, result: &str) {
    assert_eq!(
        cleanup_text(expected),
        cleanup_text(result),
        "different text in {:?}",
        path
    );
}

fn read_attributes<'a>(
    path: &mut LinkedList<&'a str>,
    tokenizer: &mut Tokenizer<'a>,
) -> Result<(BTreeMap<&'a str, &'a str>, ElementEnd<'a>), String> {
    let mut result = BTreeMap::<&'a str, &'a str>::new();
    loop {
        match tokenizer.next() {
            Some(Ok(Token::Attribute { local, value, .. })) => {
                result.insert(local.as_str(), value.as_str());
            }
            Some(Ok(Token::ElementEnd { end, .. })) => return Ok((result, end)),
            _ => return Err(format!("invalid token in attributes: {:?}", path)),
        }
    }
}

fn compare_attributes<'a>(
    path: &mut LinkedList<&'a str>,
    expected: &mut Tokenizer<'a>,
    result: &mut Tokenizer<'a>,
) -> bool {
    let (exp_attrs, exp_end) = read_attributes(path, expected).unwrap();
    let (res_attrs, res_end) = read_attributes(path, result).unwrap();

    if !exp_end.eq(&res_end) {
        panic!(
            "invalid end of element at path {:?}: {:?} <> {:?}",
            path, exp_end, res_end
        );
    }

    let exp_keys = exp_attrs.keys().copied().collect::<BTreeSet<_>>();
    let res_keys = res_attrs.keys().copied().collect::<BTreeSet<_>>();

    let diff = exp_keys.difference(&res_keys).copied().collect::<Vec<_>>();
    if !diff.is_empty() {
        panic!(
            "expected attributes {:?} but do not exist in generated: {:?}",
            diff, path
        );
    }
    let diff = res_keys.difference(&exp_keys).copied().collect::<Vec<_>>();
    if !diff.is_empty() {
        panic!(
            "generated attributes {:?} but do not exist in origin: {:?}",
            diff, path
        );
    }

    for (key, exp_value) in exp_attrs.iter() {
        if let Some(res_value) = res_attrs.get(key) {
            if key == &"style" {
                compare_style(path, exp_value, res_value);
            } else if key == &"class" {
                let exp_values = exp_value
                    .split(' ')
                    .filter(|item| !item.is_empty())
                    .collect::<BTreeSet<_>>();
                let res_values = res_value
                    .split(' ')
                    .filter(|item| !item.is_empty())
                    .collect::<BTreeSet<_>>();

                let diff = exp_values
                    .difference(&res_values)
                    .copied()
                    .collect::<BTreeSet<_>>();
                if !diff.is_empty() {
                    panic!(
                        "expected classes {:?} but do not exist in generated: {:?}",
                        diff, path
                    );
                }

                let diff = res_values
                    .difference(&exp_values)
                    .copied()
                    .collect::<BTreeSet<_>>();
                if !diff.is_empty() {
                    panic!(
                        "generated classes {:?} but do not exist in origin: {:?}",
                        diff, path
                    );
                }
            } else {
                assert_eq!(exp_value, res_value);
            }
        } else {
            panic!(
                "expected attribute {:?} in generated at path {:?}",
                key, path
            );
        }
    }

    matches!(exp_end, ElementEnd::Open)
}

fn compare_element<'a>(
    path: &mut LinkedList<&'a str>,
    expected: &mut Tokenizer<'a>,
    result: &mut Tokenizer<'a>,
) {
    let open = compare_attributes(path, expected, result);
    if open && path.back() != Some(&"meta") {
        compare_node(path, expected, result);
    } else {
        path.pop_back();
    }
}

fn next_element<'a>(tokenizer: &mut Tokenizer<'a>) -> Option<Token<'a>> {
    if let Some(token) = tokenizer.next() {
        let token = token.expect("unable to get next token");
        match token {
            Token::Text { text } => {
                if cleanup_text(text.as_str()).is_empty() {
                    next_element(tokenizer)
                } else {
                    Some(token)
                }
            }
            _ => Some(token),
        }
    } else {
        None
    }
}

fn compare_node<'a>(
    path: &mut LinkedList<&'a str>,
    expected: &mut Tokenizer<'a>,
    result: &mut Tokenizer<'a>,
) {
    loop {
        match (next_element(expected), next_element(result)) {
            (
                Some(Token::Comment { text: exp_text, .. }),
                Some(Token::Comment { text: res_text, .. }),
            ) => {
                compare_text(path, &exp_text, &res_text);
            }
            (Some(Token::Text { text: exp_text }), Some(Token::Text { text: res_text })) => {
                compare_text(path, &exp_text, &res_text);
            }
            (
                Some(Token::ElementStart { local: exp_tag, .. }),
                Some(Token::ElementStart { local: res_tag, .. }),
            ) => {
                assert_eq!(
                    exp_tag.as_str(),
                    res_tag.as_str(),
                    "tags mismatch at path {:?}",
                    path
                );
                path.push_back(exp_tag.as_str());
                compare_element(path, expected, result);
            }
            (
                Some(Token::ElementEnd { end: _exp_end, .. }),
                Some(Token::ElementEnd { end: _res_end, .. }),
            ) => {
                path.pop_back();
                // END OF ELEMENT
                return;
            }
            (Some(exp), Some(res)) => {
                panic!("mismatch at path {:?}: {:?} <> {:?}", path, exp, res);
            }
            (None, None) => {
                // nothing to do
                return;
            }
            (Some(_), None) => {
                panic!("expected element at path: {:?}", path);
            }
            (None, Some(_)) => {
                panic!("didn't expect element at path: {:?}", path);
            }
        };
    }
}

pub fn compare(expected: &str, result: &str) {
    let expected = cleanup(expected);
    let result = cleanup(result);
    println!("# expected: {}\n\n#result: {}\n", expected, result);
    let mut expected_tokenizer = Tokenizer::from(expected.as_str());
    let mut result_tokenizer = Tokenizer::from(result.as_str());
    let mut path = LinkedList::new();
    compare_node(&mut path, &mut expected_tokenizer, &mut result_tokenizer);
}
