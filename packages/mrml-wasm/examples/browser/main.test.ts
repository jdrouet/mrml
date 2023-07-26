import { Engine } from 'mrml-wasm';

describe('mrml-wasm in the browser', function () {
  it('should render to html', function () {
    const engine = new Engine();
    const result = engine.toHtml("<mjml><mj-body><mj-text>Hello world</mj-text></mj-body></mjml>");
    expect(result.type).toBe('success');
  })
});
