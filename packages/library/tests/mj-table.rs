mod common;

#[test]
fn base() {
    common::compare_render(
        include_str!("../resources/mj-table.mjml"),
        include_str!("../resources/mj-table.html"),
    );
}

#[test]
fn with_text_attributes() {
    common::compare_render(
        include_str!("../resources/mj-table-text.mjml"),
        include_str!("../resources/mj-table-text.html"),
    );
}

#[test]
fn with_table_attributes() {
    common::compare_render(
        include_str!("../resources/mj-table-table.mjml"),
        include_str!("../resources/mj-table-table.html"),
    );
}

#[test]
fn with_other_attributes() {
    common::compare_render(
        include_str!("../resources/mj-table-other.mjml"),
        include_str!("../resources/mj-table-other.html"),
    );
}
