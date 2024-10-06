import { useEffect } from "react";
import { useUpdaterStatus } from "./status"
import { invoke } from "@tauri-apps/api/core";

import "./bg.css"

export default function Updater() {
  const { text, progress } = useUpdaterStatus();

  const dark = window.matchMedia("(prefers-color-scheme: dark)").matches;

  useEffect(() => {
    window.addEventListener("keyup", (e) => {
      if (e.key == "q" || e.key == "Q") {
        invoke("launch");
      }
    })
  }, []);

  return <div data-tauri-drag-region className={`w-screen h-screen ${dark ? Math.random() >= 0.5 ? `bg1` : `bg2` : ""} flex flex-col items-center text-center justify-center py-2 pb-0`}>
    <img data-tauri-drag-region src="/favicon.png" className="h-32 w-32 mb-3 mt-auto border border-base-content rounded-full shadow-lg" />

    <h1 data-tauri-drag-region className="font-sans dark:text-white font-extrabold text-2xl md:text-3xl">Amber DChat</h1>
    <h2 data-tauri-drag-region className="text-sm font-bold dark:text-white mt-2">&#169; Amber DChat {new Date().getFullYear()}</h2>

    <h1 data-tauri-drag-region className="mt-auto pr-3 mb-2 mx-auto md:mr-0 md:ml-auto">{text}</h1>

    <progress data-tauri-drag-region max={100} value={progress} className="progress" />
  </div>
}