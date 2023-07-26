const { Engine } = require("mrml-wasm");

const engine = new Engine();
const result = engine.toHtml("<mjml><mj-body><mj-text>Hello world</mj-text></mj-body></mjml>");

if (result.type === 'success') {
  process.exit(0);
} else {
  process.exit(1);
}
