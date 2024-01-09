import wasm from 'mrml-wasm/web/mrml_wasm_bg.wasm?url';
import init, { Engine } from 'mrml-wasm/web/mrml_wasm';

export function render(input) {
  return init(wasm).then(function () {
    const engine = new Engine();
    engine.setAsyncParserOptions({
      includeLoader: {
        type: 'noop',
      },
    });
    return engine.toHtmlAsync(input);
  });
}
