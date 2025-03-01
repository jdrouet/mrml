import mrml
import os
import pytest
import sys


PARTIALS_PATH = os.path.join(os.getcwd(), 'resources', 'partials')


def test_memory_loader():
    parser_options = mrml.ParserOptions(
        include_loader=mrml.memory_loader(
            {
                "hello-world.mjml": "<mj-text>Hello World!</mj-text>",
            }
        )
    )
    result = mrml.to_html(
        '<mjml><mj-body><mj-include path="hello-world.mjml" /></mj-body></mjml>',
        parser_options=parser_options,
    )
    assert result.content.startswith("<!doctype html>")
    assert len(result.warnings) == 0


@pytest.mark.skipif(sys.platform == "win32", reason="Unable to load from local path on windows")
def test_local_loader_success():
    parser_options = mrml.ParserOptions(
        include_loader=mrml.local_loader(PARTIALS_PATH)
    )
    result = mrml.to_html(
        '<mjml><mj-body><mj-include path="file:///hello-world.mjml" /></mj-body></mjml>',
        parser_options=parser_options,
    )
    assert result.content.startswith("<!doctype html>")
    assert len(result.warnings) == 0


def test_local_loader_missing():
    parser_options = mrml.ParserOptions(
        include_loader=mrml.local_loader(PARTIALS_PATH)
    )
    try:
        mrml.to_html(
            '<mjml><mj-body><mj-include path="file:///not-found.mjml" /></mj-body></mjml>',
            parser_options=parser_options,
        )
        assert False
    except Exception as err:
        assert err


def test_http_loader_success():
    parser_options = mrml.ParserOptions(
        include_loader=mrml.http_loader(
            mode=mrml.HttpIncludeLoaderOptionsMode.Allow,
            list=set(["https://gist.githubusercontent.com"]),
        )
    )
    result = mrml.to_html(
        """<mjml>
  <mj-body>
    <mj-include
      path="https://gist.githubusercontent.com/jdrouet/b0ac80fa08a3e7262bd4c94fc8865a87/raw/ec8771f4804a6c38427ed2a9f5937e11ec2b8c27/hello-world.mjml"
    />
  </mj-body>
</mjml>""",
        parser_options=parser_options,
    )
    assert result.content.startswith("<!doctype html>")
    assert len(result.warnings) == 0


def test_http_loader_failed_not_in_allow_list():
    parser_options = mrml.ParserOptions(
        include_loader=mrml.http_loader(
            mode=mrml.HttpIncludeLoaderOptionsMode.Allow,
            list=set([]),
        )
    )
    try:
        mrml.to_html(
            """<mjml>
<mj-body>
    <mj-include
    path="https://gist.githubusercontent.com/jdrouet/b0ac80fa08a3e7262bd4c94fc8865a87/raw/ec8771f4804a6c38427ed2a9f5937e11ec2b8c27/hello-world.mjml"
    />
</mj-body>
</mjml>""",
            parser_options=parser_options,
        )
        assert False
    except Exception as err:
        assert err


def test_http_loader_success_allow_everything():
    parser_options = mrml.ParserOptions(
        include_loader=mrml.http_loader(mode=mrml.HttpIncludeLoaderOptionsMode.Deny)
    )
    mrml.to_html(
        """<mjml>
<mj-body>
    <mj-include
    path="https://gist.githubusercontent.com/jdrouet/b0ac80fa08a3e7262bd4c94fc8865a87/raw/ec8771f4804a6c38427ed2a9f5937e11ec2b8c27/hello-world.mjml"
    />
</mj-body>
</mjml>""",
        parser_options=parser_options,
    )


def test_http_loader_failed_deny_github():
    parser_options = mrml.ParserOptions(
        include_loader=mrml.http_loader(
            mode=mrml.HttpIncludeLoaderOptionsMode.Deny,
            list=set(["https://gist.githubusercontent.com"]),
        )
    )
    try:
        mrml.to_html(
            """<mjml>
<mj-body>
    <mj-include
    path="https://gist.githubusercontent.com/jdrouet/b0ac80fa08a3e7262bd4c94fc8865a87/raw/ec8771f4804a6c38427ed2a9f5937e11ec2b8c27/hello-world.mjml"
    />
</mj-body>
</mjml>""",
            parser_options=parser_options,
        )
        assert False
    except Exception as err:
        assert err
