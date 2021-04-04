use html_parser::{Dom, Element, Node};
use std::collections::HashSet;

fn trim_header_comment(input: &str) -> String {
    if input.starts_with("<!-- FILE:") {
        if let Some(index) = input.find("\n") {
            return input.split_at(index).1.to_string();
        }
    }
    input.to_string()
}

fn cleanup(input: &str) -> String {
    trim_header_comment(input)
        // conditions and comments
        .replace("<!--[if !mso]><!-- -->", "")
        .replace("<!--[if !mso><!-->", "")
        .replace("<!--[if !mso]><!-->", "")
        .replace("<!--<![endif]-->", "")
        .replace("<!--[if mso | IE]>", "")
        .replace("<!--[if !mso | IE]>", "")
        .replace("<!--[if mso]>", "")
        .replace("<!--[if !mso]>", "")
        .replace("<!--[if lte mso 11]>", "")
        .replace("<!--<![endif]-->", "")
        .replace("<![endif]-->", "")
        .replace("<!-->", "")
        // empty style header blocks
        .replace("<style type=\"text/css\">\n  </style>", "")
        // empty style attributes
        .replace("style=\"\"", "")
        // empty class attributes
        .replace("class=\"\"", "")
        // empty divs
        .replace("<div></div>", "")
}

pub fn cleanup_text(input: &str) -> String {
    input.replace(" ", "").replace("\n", "")
}

fn compare_style(path: &str, expected: &str, result: &str) {
    let expected_set = expected
        .split(';')
        .filter(|item| !item.is_empty())
        .map(|item| item.to_string())
        .collect::<HashSet<String>>();
    let result_set = result
        .split(';')
        .filter(|item| !item.is_empty())
        .map(|item| item.to_string())
        .collect::<HashSet<String>>();
    assert_eq!(expected_set, result_set, "different styles in {}", path);
}

fn compare_attribute(path: &str, key: &str, expected: Option<&String>, result: Option<&String>) {
    let expected = expected.cloned().unwrap_or_default();
    let result = result.cloned().unwrap_or_default();
    if key == "style" {
        compare_style(path, &expected, &result);
    } else {
        assert_eq!(expected, result, "different attribute {} in {}", key, path);
    }
}

fn compare_element(path: &str, expected: &Element, result: &Element) {
    assert_eq!(expected.name, result.name, "different element in {}", path);
    let current_path = format!("{} {}.{}", path, result.name, result.classes.join("."));
    assert_eq!(expected.id, result.id, "different id in {}", current_path);
    for (key, value) in expected.attributes.iter() {
        compare_attribute(
            current_path.as_str(),
            key,
            value.as_ref(),
            result.attributes.get(key).and_then(|v| v.as_ref()),
        );
    }
    assert_eq!(
        expected.classes.len(),
        result.classes.len(),
        "classes mismatch in {}",
        current_path
    );
    for classname in expected.classes.iter() {
        assert!(
            result.classes.contains(classname),
            "cannot find classname {} in {}",
            classname,
            current_path
        );
    }
    if expected.children.len() != result.children.len() {
        println!("expected: {:#?}", expected.children);
        println!("result: {:#?}", result.children);
    }
    assert_eq!(
        expected.children.len(),
        result.children.len(),
        "children count mismatch in {}",
        current_path
    );
    for (index, (expected_child, result_child)) in expected
        .children
        .iter()
        .zip(result.children.iter())
        .enumerate()
    {
        let iterate_path = format!("{}[{}]", current_path, index);
        compare_node(iterate_path.as_str(), expected_child, result_child);
    }
}

fn compare_text(path: &str, expected: &str, result: &str) {
    assert_eq!(
        cleanup_text(expected),
        cleanup_text(result),
        "different text in {}",
        path
    );
}

fn compare_node(path: &str, expected: &Node, result: &Node) {
    if let Node::Element(exp) = expected {
        if let Node::Element(res) = result {
            return compare_element(path, &exp, &res);
        }
    }
    if let Node::Text(exp) = expected {
        if let Node::Text(res) = result {
            return compare_text(path, &exp, &res);
        }
    }
    assert_eq!(expected, result);
}

fn compare_dom(expected: &Dom, result: &Dom) {
    assert_eq!(expected.tree_type, result.tree_type);
    assert_eq!(expected.children.len(), result.children.len());
    for (expected_child, result_child) in expected.children.iter().zip(result.children.iter()) {
        compare_node("", expected_child, result_child);
    }
}

pub fn compare(expected: &str, result: &str) {
    let expected = cleanup(expected);
    let result = cleanup(result);
    let expected_dom = Dom::parse(&expected).unwrap();
    let result_dom = Dom::parse(&result).unwrap();
    compare_dom(&expected_dom, &result_dom);
}
