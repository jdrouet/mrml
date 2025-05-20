import mrml

def test_simple_template():
    result = mrml.to_html("<mjml></mjml>")
    assert result.content.startswith("<!doctype html>")
    assert len(result.warnings) == 0

def test_with_warnings():
    result = mrml.to_html("<mjml yolo=\"foo\"></mjml>")
    assert result.content.startswith("<!doctype html>")
    assert len(result.warnings) == 1
    assert result.warnings[0].kind == "unexpected-attribute"
    assert result.warnings[0].start == 6
    assert result.warnings[0].end == 16

def test_template_with_options():
    parser_options = mrml.ParserOptions(include_loader = mrml.memory_loader({
        'hello-world.mjml': '<mj-text>Hello World!</mj-text>',
    }))
    result = mrml.to_html("<mjml><mj-body><mj-include path=\"hello-world.mjml\" /></mj-body></mjml>", parser_options = parser_options)
    assert result.content.startswith("<!doctype html>")

def test_title_present():
    result = mrml.to_html("""<mjml>
        <mj-head>
            <mj-title>Hello MJML</mj-title>
        </mj-head>
    </mjml>""")
    assert result.title == "Hello MJML"

def test_title_absent():
    result = mrml.to_html("<mjml></mjml>")
    assert result.title is None

def test_preview_present():
    result = mrml.to_html("""<mjml>
        <mj-head>
            <mj-preview>Hello MJML</mj-preview>
        </mj-head>
    </mjml>""")
    assert result.preview == "Hello MJML"

def test_preview_absent():
    result = mrml.to_html("<mjml></mjml>")
    assert result.preview is None
