import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { ProgressBarStatus } from "@tauri-apps/api/window";
import { useEffect, useState } from "react";

export function useUpdaterStatus() {
  const [text, setText] = useState("Checking for Updates...");
  const [progress, setProgress] = useState<number | undefined>(undefined);

  useEffect(() => {
    const window = getCurrentWebviewWindow();

    window.setProgressBar({
      status: ProgressBarStatus.Indeterminate
    });

    invoke("check_update")
      .catch(() => {
        window.emit("update", "error");
      });

    window.listen("update", ({ payload }) => {
      if (payload == "error") {
        setText("Error, trying again...");

        window.setProgressBar({
          status: ProgressBarStatus.Error,
          progress: 50
        });

        setTimeout(() => {
          invoke("check_update")
            .catch(() => {
              window.emit("update", "error");
            })
        }, 5000);
      } else if (payload === "none") {
        setText("Launching...");

        window.setProgressBar({
          status: ProgressBarStatus.None
        });

        setTimeout(() => {
          invoke("launch");
        }, 1000);
      } else if (payload === "Installing") {
        setText("Installing...");
        window.setProgressBar({
          status: ProgressBarStatus.Indeterminate
        });
      } else if (payload === "Installed") {
        setText("Installed");
        window.setProgressBar({
          status: ProgressBarStatus.None
        });
      } else {
        const progress = payload as number;
        setProgress(progress);

        window.setProgressBar({
          status: ProgressBarStatus.Normal,
          progress: progress
        });
      }
    });
  }, []);

  return {
    text,
    progress
  }
}