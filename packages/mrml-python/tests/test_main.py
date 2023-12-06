import mrml

def test_simple_template():
    result = mrml.to_html("<mjml></mjml>")
    assert result.startswith("<!doctype html>")

def test_template_with_options():
    parser_options = mrml.ParserOptions(include_loader = mrml.memory_loader({
        'hello-world.mjml': '<mj-text>Hello World!</mj-text>',
    }))
    result = mrml.to_html("<mjml><mj-body><mj-include path=\"hello-world.mjml\" /></mj-body></mjml>", parser_options = parser_options)
    assert result.startswith("<!doctype html>")
