const wasm = import("../pkg/index.js");

const initial = `<mjml>
  <mj-body>
    <mj-section>
      <mj-column>
        <mj-text align="center">
          Welcome!
        </mj-text>
        <mj-image border-radius="5px" src="http://placekitten.com/500/300" />
        <mj-button>
          Click me!
        </mj-button>
      </mj-column>
    </mj-section>
  </mj-body>
</mjml>`;

const render = (input) => wasm.then((bin) => bin.to_html(input));

const setTextareaResult = (value) => {
  const result = document.querySelector('textarea[name="result"]');
  result.value = value;
};

const setIFrameResult = (value) => {
  const iframe = document.querySelector('iframe[name="render"]');
  iframe.setAttribute("srcdoc", value);
};

const setResult = (html) => {
  setIFrameResult(html);
  setTextareaResult(html);
};

const onSourceChange = (event) => {
  render(event.target.value).then(setResult);
};

const run = () => {
  const source = document.querySelector('textarea[name="source"]');
  source.value = initial;
  onSourceChange({ target: { value: initial } });
  source.addEventListener("change", onSourceChange);
};

window.addEventListener("load", run);
