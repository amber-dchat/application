import ReactDOM from "react-dom/client";
import Updater from "./ui";

import { invoke } from "@tauri-apps/api/core";

import "./main.css"

invoke("ready");

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <Updater />,
);
