import React from "react";
import ReactDOM from "react-dom/client";

import "./main.css"
import Updater from "./ui";
import { invoke } from "@tauri-apps/api/core";

invoke("ready");

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Updater />
  </React.StrictMode>,
);
