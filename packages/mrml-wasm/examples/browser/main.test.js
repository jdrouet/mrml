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
    expect(result.type).toBe('success');
    expect(result.content).not.toContain('Goodbye');
  });

  it('should use noop include loader by default', function () {
    const engine = new Engine();
    engine.setRenderOptions({
      disableComments: true,
      fonts: {},
    });
    const result = engine.toHtml(`<mjml>
<mj-body>
  <mj-include path="./header.mjml" />
</mj-body>
</mjml>`);
    expect(result.type).toBe('error');
  });

  it('should use memory include loader', function () {
    const engine = new Engine();
    engine.setParserOptions({
      includeLoader: {
        type: 'memory',
        content: {
          './header.mjml': '<mj-text>Hello World</mj-text>',
        },
      },
    });
    engine.setRenderOptions({
      disableComments: true,
      fonts: {},
    });
    const result = engine.toHtml(`<mjml>
<mj-body>
  <mj-include path="./header.mjml" />
</mj-body>
</mjml>`);
    expect(result.type).toBe('success');
    expect(result.content).toContain('Hello World');
  });
});
