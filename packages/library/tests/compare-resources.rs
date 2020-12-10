use difference::assert_diff;
use html_parser::{Dom, Element, Node};
use mrml::{to_html, to_preview, to_title, Options};
use std::collections::HashMap;
use test_generator::test_resources;

fn compare_str(result: &str, expected: &str) {
    if std::env::var("CI").is_ok() {
        assert_eq!(expected, result);
    } else {
        assert_diff!(expected, result, "", 0);
    }
}

fn clean_text(input: &str) -> String {
    input
        .replace("\n", " ")
        .replace("\t", "")
        .split_whitespace()
        .collect::<String>()
}

fn compare_text(_path: &str, result: &str, expected: &str) {
    let result = clean_text(result);
    let expected = clean_text(expected);
    compare_str(result.as_str(), expected.as_str());
}

fn clean_attribute(input: &str) -> String {
    input.split_whitespace().collect::<String>()
}

fn compare_attribute(_path: &str, result: &str, expected: &str) {
    let result = clean_attribute(result);
    let expected = clean_attribute(expected);
    compare_str(result.as_str(), expected.as_str());
}

fn filter_empty_attributes(entry: &(&String, &Option<String>)) -> bool {
    if let Some(value) = entry.1 {
        !value.trim().is_empty()
    } else {
        false
    }
}

fn compare_attributes(
    path: &str,
    result: &HashMap<String, Option<String>>,
    expected: &HashMap<String, Option<String>>,
) {
    let mut result = result
        .iter()
        .filter(filter_empty_attributes)
        .collect::<Vec<_>>();
    let mut expected = expected
        .iter()
        .filter(filter_empty_attributes)
        .collect::<Vec<_>>();
    result.sort();
    expected.sort();
    for (result_entry, expected_entry) in result.iter().zip(expected.iter()) {
        assert_eq!(
            result_entry.0, expected_entry.0,
            "{} attributes mismatch",
            path
        );
        let path = format!("{} [{}]", path, result_entry.0);
        let result_value = result_entry.1.as_ref().unwrap();
        let expected_value = expected_entry.1.as_ref().unwrap();
        compare_attribute(
            path.as_str(),
            result_value.as_str(),
            expected_value.as_str(),
        )
    }
}

fn compare_classes(path: &str, result: &Vec<String>, expected: &Vec<String>) {
    let mut result = result.clone();
    let mut expected = expected.clone();
    result.sort();
    expected.sort();
    assert_eq!(result, expected, "{} classes mismatch", path);
}

fn compare_element(path: &str, result: &Element, expected: &Element) {
    assert_eq!(result.name, expected.name, "{} name mismatch", path);
    assert_eq!(result.id, expected.id, "{} id mismatch", path);
    compare_classes(path, &result.classes, &expected.classes);
    compare_attributes(path, &result.attributes, &expected.attributes);
    compare_children(path, &result.children, &expected.children);
}

fn compare_child(path: &str, result: &Node, expected: &Node) {
    match result {
        Node::Text(result_text) => match expected {
            Node::Text(expected_text) => {
                compare_text(path, result_text.as_str(), expected_text.as_str())
            }
            _ => panic!("{} type mismatch", path),
        },
        Node::Comment(result_text) => match expected {
            Node::Comment(expected_text) => {
                compare_text(path, result_text.as_str(), expected_text.as_str())
            }
            _ => panic!("{} type mismatch", path),
        },
        Node::Element(result_element) => match expected {
            Node::Element(expected_element) => {
                compare_element(path, result_element, expected_element)
            }
            _ => panic!("{} type mismatch", path),
        },
    }
}

fn compare_children(path: &str, result: &Vec<Node>, expected: &Vec<Node>) {
    for (i, (result_child, expected_child)) in result.iter().zip(expected.iter()).enumerate() {
        let path = format!("{} > {}", path, i);
        compare_child(path.as_str(), result_child, expected_child);
    }
}

fn compare_html(result: &str, expected: &str) {
    let result = Dom::parse(result).unwrap();
    let expected = Dom::parse(expected).unwrap();
    if result.tree_type != expected.tree_type {
        panic!("tree types are different");
    }
    compare_children("root", &result.children, &expected.children);
}

fn generate_id(size: usize) -> String {
    println!("test generate id");
    std::iter::repeat(())
        .map(|()| 'a')
        .take(size)
        .collect::<String>()
}

fn build_options() -> Options {
    let mut opts = Options::default();
    opts.id_generator = generate_id;
    opts
}

fn get_template_name(mjml_file: &str) -> String {
    mjml_file.strip_suffix(".mjml").map(String::from).unwrap()
}

fn get_mjml_content(name: &str) -> Option<String> {
    std::fs::read_to_string(format!("{}.mjml", name)).ok()
}

fn get_html_content(name: &str) -> Option<String> {
    std::fs::read_to_string(format!("{}.html", name)).ok()
}

#[test_resources("resources/compare/success/*.mjml")]
fn compare_success(mjml_file: &str) {
    let name = get_template_name(mjml_file);
    let mjml_content = get_mjml_content(name.as_str()).unwrap();
    let html_content = match get_html_content(name.as_str()) {
        Some(value) => value,
        None => {
            println!("couldn't find html for {}", name);
            return;
        }
    };
    let result = to_html(mjml_content.as_str(), build_options()).unwrap();
    compare_html(result.as_str(), html_content.as_str());
}

#[test_resources("resources/compare/error/*.mjml")]
fn compare_error(mjml_file: &str) {
    let name = get_template_name(mjml_file);
    let mjml_content = get_mjml_content(name.as_str()).unwrap();
    if let Ok(_) = to_html(mjml_content.as_str(), build_options()) {
        panic!("should panic");
    }
}

#[test]
fn mj_social_with_different_origin() {
    let name = "resources/compare/success/mj-social";
    let mjml_content = get_mjml_content(name).unwrap();
    let html_content = get_html_content(name).unwrap();
    let html_content = html_content.replace(
        "https://www.mailjet.com/images/theme/v1/icons/ico-social/",
        "http://my.origin.rust/",
    );
    let mut opts = build_options();
    opts.social_icon_origin = String::from("http://my.origin.rust/");
    let result = to_html(mjml_content.as_str(), opts).unwrap();
    compare_html(result.as_str(), html_content.as_str());
}

#[test]
fn mj_body_without_comments() {
    let mjml_content = get_mjml_content("resources/compare/success/mj-body").unwrap();
    let html_content = get_html_content("resources/compare/mj-body-without-comments").unwrap();
    let mut opts = build_options();
    opts.keep_comments = false;
    let result = to_html(mjml_content.as_str(), opts).unwrap();
    compare_html(result.as_str(), html_content.as_str());
}

#[test]
#[should_panic]
fn mj_body_with_unknown_tag() {
    let mjml_content =
        std::fs::read_to_string("resources/compare/error/mj-head-unknown-tag.mjml").unwrap();
    to_html(mjml_content.as_str(), build_options()).unwrap();
}

#[test]
fn mj_preview() {
    let mjml_content = get_mjml_content("resources/compare/success/mj-preview").unwrap();
    let result = to_preview(mjml_content.as_str(), build_options()).unwrap();
    assert_diff!(result.as_str(), "Hello MJML", "", 0);
}

#[test]
fn mj_title() {
    let mjml_content = get_mjml_content("resources/compare/success/mj-title").unwrap();
    let result = to_title(mjml_content.as_str(), build_options()).unwrap();
    assert_diff!(result.as_str(), "Hello MJML", "", 0);
}
