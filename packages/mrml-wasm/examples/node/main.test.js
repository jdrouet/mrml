const assert = require("assert");
const { Engine } = require("mrml-wasm");
const { describe, it } = require("node:test");

describe('mrml-wasm in node', function () {
  it('should render to html', function () {
    const engine = new Engine();
    const result = engine.toHtml("<mjml><mj-body><mj-text>Hello world</mj-text></mj-body></mjml>");
    assert.equal(result.type, 'success');
  });

  it('should disable the comments', function () {
    const engine = new Engine();
    engine.setRenderOptions({
      disableComments: true,
      fonts: {},
    });
    const result = engine.toHtml(`<mjml>
  <mj-body>
    <!-- Hello -->
    <mj-text>Hello world</mj-text>
    <!-- Goodbye -->
  </mj-body>
</mjml>`);
    assert.equal(result.type, 'success');
    assert.doesNotMatch(result.content, /Goodbye/);
  });
});
