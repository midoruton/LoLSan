import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { attachConsole } from "@tauri-apps/plugin-log";

//https://v2.tauri.app/plugin/logging/#logging-to-the-webview-console
attachConsole();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
