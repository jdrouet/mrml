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

fn compare_style(expected: &str, result: &str) {
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
    assert_eq!(expected_set, result_set);
}

fn compare_attribute(key: &str, expected: Option<&String>, result: Option<&String>) {
    let expected = expected.cloned().unwrap_or_default();
    let result = result.cloned().unwrap_or_default();
    if key == "style" {
        compare_style(&expected, &result);
    } else {
        assert_eq!(expected, result);
    }
}

fn compare_element(expected: &Element, result: &Element) {
    assert_eq!(expected.name, result.name);
    assert_eq!(expected.id, result.id);
    for (key, value) in expected.attributes.iter() {
        compare_attribute(
            key,
            value.as_ref(),
            result.attributes.get(key).and_then(|v| v.as_ref()),
        );
    }
    for classname in expected.classes.iter() {
        assert!(result.classes.contains(classname));
    }
    for (expected_child, result_child) in expected.children.iter().zip(result.children.iter()) {
        compare_node(expected_child, result_child);
    }
}

fn compare_text(expected: &str, result: &str) {
    assert_eq!(cleanup_text(expected), cleanup_text(result));
}

fn compare_node(expected: &Node, result: &Node) {
    if let Node::Element(exp) = expected {
        if let Node::Element(res) = result {
            return compare_element(&exp, &res);
        }
    }
    if let Node::Text(exp) = expected {
        if let Node::Text(res) = result {
            return compare_text(&exp, &res);
        }
    }
    assert_eq!(expected, result);
}

fn compare_dom(expected: &Dom, result: &Dom) {
    assert_eq!(expected.tree_type, result.tree_type);
    assert_eq!(expected.children.len(), result.children.len());
    for (expected_child, result_child) in expected.children.iter().zip(result.children.iter()) {
        compare_node(expected_child, result_child);
    }
}

pub fn compare(expected: &str, result: &str) {
    let expected_dom = Dom::parse(expected).unwrap();
    let result_dom = Dom::parse(result).unwrap();
    compare_dom(&expected_dom, &result_dom);
}
