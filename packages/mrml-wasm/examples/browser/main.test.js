import { Engine } from 'mrml-wasm';

describe('mrml-wasm in the browser', function () {
  it('should render to html', function () {
    const engine = new Engine();
    const result = engine.toHtml("<mjml><mj-body><mj-text>Hello world</mj-text></mj-body></mjml>");
    expect(result.type).toBe('success');
  })

  it('should disable the comments', function () {
    const engine = new Engine();
    engine.setRenderOptions({
      disable_comments: true,
      fonts: {},
    });
    const result = engine.toHtml(`<mjml>
  <mj-body>
    <!-- Hello -->
    <mj-text>Hello world</mj-text>
    <!-- Goodbye -->
  </mj-body>
</mjml>`);
    expect(result.type).toBe('success');
    expect(result.content).not.toContain('Goodbye');
  });
});
