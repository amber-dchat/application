use tauri::{AppHandle, Emitter, Manager, Url, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

#[cfg(desktop)]
use tauri_plugin_updater::UpdaterExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default();

    #[cfg(not(mobile))]
    let builder = builder
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            if let Some((_, win)) = app.webview_windows().iter().next() {
                let _ = win.set_focus();
            }
        }));

    builder
        .invoke_handler(tauri::generate_handler![ready, launch, check_update])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn ready(window: WebviewWindow) {
    let _ = window.show();
}

#[tauri::command]
#[allow(unused_mut)]
async fn launch(mut window: WebviewWindow, _app: AppHandle) {
    #[cfg(mobile)]
    {
        window.navigate(Url::parse("https://dchat-app.github.io").unwrap());
    }

    #[cfg(desktop)]
    {
        let w = WebviewWindowBuilder::new(&_app, "chatapplication", WebviewUrl::External(Url::parse("https://dchat-app.github.io").unwrap()))
            .title("DChatt")
            .center()
            .min_inner_size(1024.0, 768.0)
            .closable(true)
            .resizable(true)
            .maximized(true)
            .build()
            .unwrap();
        
        let _ = w.set_focus();
        let _ = w.maximize();
        let _ = w.set_focus();

        let _ = window.destroy();
    }
}

#[tauri::command]
#[cfg(mobile)]
async fn check_update() -> Result<(), &'static str> {
    Err("Updater not implemented")
}

#[tauri::command]
#[cfg(desktop)]
async fn check_update(app: AppHandle) -> tauri_plugin_updater::Result<()> {
    let window = app.get_webview_window("splash").unwrap();
    if let Some(update) = app.updater()?.check().await? {
        update.download_and_install(|c, t| {
            let c = c as u64;
            let total = t.unwrap_or(1);

            let _ = window.emit("update", (c * 100) / total);
        }, || {
            let _ = window.emit("update", "Installing");
        }).await?;

        let _ = window.emit("update", "Installed");
        app.restart();
    }

    let _ = window.emit("update", "none");

    Ok(())
}