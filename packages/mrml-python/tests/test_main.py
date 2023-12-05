import mrml

def test_simple_template():
    result = mrml.to_html("<mjml></mjml>")
    assert result.startswith("<!doctype html>")
