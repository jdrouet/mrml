import concurrent.futures

import mrml
import os
import pytest
import sys


PARTIALS_PATH = os.path.join(os.getcwd(), 'resources', 'partials')


def test_concurrent_simple_template():
    def worker():
        result = mrml.to_html("<mjml></mjml>")
        assert result.content.startswith("<!doctype html>")
        assert len(result.warnings) == 0
        return True

    with concurrent.futures.ThreadPoolExecutor(max_workers=10) as executor:
        futures = [executor.submit(worker) for _ in range(100)]
        results = [f.result() for f in futures]
        assert all(results)


def test_concurrent_memory_loader():
    parser_options = mrml.ParserOptions(
        include_loader=mrml.memory_loader(
            {
                "hello-world.mjml": "<mj-text>Hello World!</mj-text>",
            }
        )
    )

    def worker():
        result = mrml.to_html(
            '<mjml><mj-body><mj-include path="hello-world.mjml" /></mj-body></mjml>',
            parser_options=parser_options,
        )
        assert result.content.startswith("<!doctype html>")
        assert len(result.warnings) == 0
        return True

    with concurrent.futures.ThreadPoolExecutor(max_workers=10) as executor:
        futures = [executor.submit(worker) for _ in range(100)]
        results = [f.result() for f in futures]
        assert all(results)


@pytest.mark.skipif(sys.platform == "win32", reason="Unable to load from local path on windows")
def test_concurrent_local_loader():
    parser_options = mrml.ParserOptions(
        include_loader=mrml.local_loader(PARTIALS_PATH)
    )

    def worker():
        result = mrml.to_html(
            '<mjml><mj-body><mj-include path="file:///hello-world.mjml" /></mj-body></mjml>',
            parser_options=parser_options,
        )
        assert result.content.startswith("<!doctype html>")
        assert len(result.warnings) == 0
        return True

    with concurrent.futures.ThreadPoolExecutor(max_workers=10) as executor:
        futures = [executor.submit(worker) for _ in range(100)]
        results = [f.result() for f in futures]
        assert all(results)


def test_concurrent_http_loader():
    http_loader = mrml.http_loader(
        mode=mrml.HttpIncludeLoaderOptionsMode.Allow,
        list=set(["https://gist.githubusercontent.com"]),
    )
    parser_options = mrml.ParserOptions(include_loader=http_loader)

    def worker():
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
        return True

    with concurrent.futures.ThreadPoolExecutor(max_workers=10) as executor:
        futures = [
            executor.submit(worker) for _ in range(20)
        ]  # Fewer iterations for HTTP test
        results = [f.result() for f in futures]
        assert all(results)


@pytest.mark.skipif(sys.platform == "win32", reason="Unable to load from local path on windows")
def test_concurrent_mixed_operations():
    """Test different MRML operations running concurrently"""
    memory_parser_options = mrml.ParserOptions(
        include_loader=mrml.memory_loader(
            {
                "hello-world.mjml": "<mj-text>Hello World!</mj-text>",
            }
        )
    )

    local_parser_options = mrml.ParserOptions(
        include_loader=mrml.local_loader(PARTIALS_PATH)
    )

    def worker_simple():
        result = mrml.to_html("<mjml></mjml>")
        assert result.content.startswith("<!doctype html>")
        return "simple"

    def worker_memory():
        result = mrml.to_html(
            '<mjml><mj-body><mj-include path="hello-world.mjml" /></mj-body></mjml>',
            parser_options=memory_parser_options,
        )
        assert result.content.startswith("<!doctype html>")
        return "memory"

    def worker_local():
        result = mrml.to_html(
            '<mjml><mj-body><mj-include path="file:///hello-world.mjml" /></mj-body></mjml>',
            parser_options=local_parser_options,
        )
        assert result.content.startswith("<!doctype html>")
        return "local"

    workers = [worker_simple, worker_memory, worker_local]

    with concurrent.futures.ThreadPoolExecutor(max_workers=10) as executor:
        task_count = 100
        futures = []
        for _ in range(task_count):  # 30 total operations
            for worker in workers:
                futures.append(executor.submit(worker))

        results = [f.result() for f in futures]
        assert len(results) == task_count * len(workers)
        assert all(r in ["simple", "memory", "local"] for r in results)


def test_render_options_thread_safety():
    """Test mutation throws AttributeError"""
    render_options = mrml.RenderOptions(disable_comments=True)
    assert render_options.disable_comments
    assert render_options.social_icon_origin is None
    with pytest.raises(AttributeError) as _:
        render_options.disable_comments = False
