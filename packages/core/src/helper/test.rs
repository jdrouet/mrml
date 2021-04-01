use html_parser::{Dom, Element, Node};
use std::collections::HashSet;

pub fn cleanup(input: &str) -> String {
    input
        .replace(" ", "")
        .replace("\n", "")
        .replace("<styletype=\"text/css\"></style>", "")
        .replace("style=\"\"", "")
        .replace("class=\"\"", "")
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
    assert_eq!(expected.name, result.name);
    assert_eq!(expected.id, result.id);
    let current_path = format!("{} > {}", path, result.name);
    for (key, value) in expected.attributes.iter() {
        compare_attribute(
            current_path.as_str(),
            key,
            value.as_ref(),
            result.attributes.get(key).and_then(|v| v.as_ref()),
        );
    }
    assert_eq!(expected.classes.len(), result.classes.len());
    for classname in expected.classes.iter() {
        assert!(
            result.classes.contains(classname),
            "cannot find classname {} in {}",
            classname,
            current_path
        );
    }
    for (expected_child, result_child) in expected.children.iter().zip(result.children.iter()) {
        compare_node(current_path.as_str(), expected_child, result_child);
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
    let expected_dom = Dom::parse(expected).unwrap();
    let result_dom = Dom::parse(result).unwrap();
    compare_dom(&expected_dom, &result_dom);
}
