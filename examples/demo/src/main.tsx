import React from "react";
import ReactDOM from "react-dom/client";
import { Application } from "./application.tsx";
import "./index.css";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <Application />
  </React.StrictMode>
);
