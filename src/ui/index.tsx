import { useEffect } from "react";
import { useUpdaterStatus } from "./status"
import { invoke } from "@tauri-apps/api/core";

export default function Updater() {
  const { text, progress } = useUpdaterStatus();

  useEffect(() => {
    window.addEventListener("keyup", (e) => {
      if (e.key == "q" || e.key == "Q") {
        invoke("launch");
      }
    })
  }, []);

  return <div data-tauri-drag-region className="w-screen h-screen flex flex-col items-center text-center justify-center px-10 py-2">
    <img data-tauri-drag-region src="/favicon.png" className="h-32 w-32 mb-3 mt-auto" />

    <h1 data-tauri-drag-region className="font-sans font-extrabold text-lg">{text}</h1>

    <progress data-tauri-drag-region max={100} value={progress} className="progress progress-info mt-3" />

    <h2 data-tauri-drag-region className="text-sm font-extrabold mt-auto">&#169; DChatt {new Date().getFullYear()}</h2>
  </div>
}