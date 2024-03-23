import React from "react";
import Editor from "@monaco-editor/react";
import { Engine, ToHtmlResult } from "mrml/bundler/mrml_wasm";
import "./application.css";

const DEFAULT_TEMPLATE = `<mjml>
  <mj-body>
    <mj-text>Hello World</mj-text>
  </mj-body>
</mjml>`;

export const Application = () => {
  const engine = React.useMemo(() => new Engine(), []);
  const [template, setTemplate] = React.useState(DEFAULT_TEMPLATE);
  const [output, setOutput] = React.useState<ToHtmlResult>({
    type: "success",
    content: "",
  });

  React.useEffect(() => {
    engine.toHtmlAsync(template).then(setOutput);
  }, [engine, template, setOutput]);

  return (
    <>
      <Editor
        className="editor"
        height="auto"
        width="50%"
        defaultLanguage="XML"
        defaultValue={DEFAULT_TEMPLATE}
        onChange={(value) => {
          if (value) {
            setTemplate(value);
          }
        }}
      />
      {output.type === "success" ? (
        <iframe className="preview" srcDoc={output.content} />
      ) : (
        <section className="preview" />
      )}
    </>
  );
};
