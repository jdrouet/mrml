import { Engine } from 'mrml-wasm/bundler/mrml_wasm';

export function render(input) {
  const engine = new Engine();
  engine.setAsyncParserOptions({
    includeLoader: {
      type: 'noop',
    },
  });
  return engine.toHtmlAsync(input);
}
