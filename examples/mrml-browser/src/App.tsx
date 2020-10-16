import React from "react";
import "./App.css";

function App() {
  const [mrml, setMrml] = React.useState<any>();
  const [template, setTemplate] = React.useState<string>("<mjml></mjml>");
  const [result, setResult] = React.useState<string>("");
  React.useEffect(() => {
    import("mrml/bundler/mrml").then(setMrml);
  }, [setMrml]);
  React.useEffect(() => {
    if (!mrml) return;
    try {
      setResult(mrml.toHtml(template));
    } catch (err) {
      console.error(err);
    }
  }, [mrml, template, setResult]);
  return (
    <main>
      <textarea
        value={template}
        onChange={(e) => setTemplate(e.target.value)}
      />
      <textarea value={result} readOnly />
    </main>
  );
}

export default App;
