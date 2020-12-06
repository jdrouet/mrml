use difference::assert_diff;
use std::iter;

use mrml::{to_html, to_preview, to_title, Options};

pub fn generate_id(size: usize) -> String {
    println!("test generate id");
    iter::repeat(())
        .map(|()| 'a')
        .take(size)
        .collect::<String>()
}

pub fn build_options() -> Options {
    let mut opts = Options::default();
    opts.id_generator = generate_id;
    opts
}

fn should_use_equal() -> bool {
    std::env::var("CI").is_ok()
}

fn diff(first: &str, second: &str) {
    if should_use_equal() {
        assert_eq!(first, second);
    } else {
        assert_diff!(first, second, "", 0);
    }
}

fn clean_str(input: String) -> String {
    input
        .replace("class=\"\"", "")
        .replace("style=\"\"", "")
        .replace("\n", "")
        .replace("\t", "")
        .replace(" ", "")
        .replace("<![endif]--><!--[ifmso|IE]>", "")
}

pub fn compare_title(source: &str, expected: &str) {
    let result = to_title(source, build_options()).unwrap();
    assert_diff!(result.as_str(), expected, "", 0);
}

pub fn compare_preview(source: &str, expected: &str) {
    let result = to_preview(source, build_options()).unwrap();
    assert_diff!(result.as_str(), expected, "", 0);
}

pub fn compare_render(source: &str, expected: &str) {
    let result = to_html(source, build_options()).unwrap();
    let result = clean_str(result);
    let expected = clean_str(expected.into());
    diff(result.as_str(), expected.as_str());
}

pub fn compare_render_with_options(source: &str, expected: &str, opts: Options) {
    let result = to_html(source, opts).unwrap();
    let result = clean_str(result);
    let expected = clean_str(expected.into());
    diff(result.as_str(), expected.as_str());
}
