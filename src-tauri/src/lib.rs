use tauri::{AppHandle, Emitter, Manager, Url, WebviewWindow};

#[cfg(desktop)]
use tauri::{WebviewUrl, WebviewWindowBuilder};

#[cfg(mobile)]
use ahq_updater as updater;

#[cfg(desktop)]
use tauri_plugin_updater as updater;

#[cfg(desktop)]
use rpc_server::bootstrap;

#[cfg(desktop)]
mod structs;

use updater::UpdaterExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(updater::Builder::new().build());

    #[cfg(not(mobile))]
    let builder = builder
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
#[cfg(mobile)]
fn ready() {}

#[tauri::command]
#[cfg(desktop)]
fn ready(window: WebviewWindow) {
    let _ = window.show();
}

#[tauri::command]
#[allow(unused_mut)]
async fn launch(mut window: WebviewWindow, _app: AppHandle) {
    #[cfg(mobile)]
    {
        let _ = window.navigate(Url::parse("https://amber-dchat.github.io/").unwrap());
    }

    #[cfg(desktop)]
    {
        #[cfg(debug_assertions)]
        _app.add_capability(include_str!("../capabilities/debug.json"));

        #[cfg(debug_assertions)]
        let url = WebviewUrl::External(Url::parse("http://localhost:3000/").unwrap());
        
        #[cfg(not(debug_assertions))]
        let url = WebviewUrl::External(Url::parse("https://amber-dchat.github.io/").unwrap());

        let w = WebviewWindowBuilder::new(&_app, "chatapplication", url)
            .title("Amber DChat")
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

        bootstrap(structs::IWindow { inner: w });

        let _ = window.destroy();
    }
}

#[tauri::command]
async fn check_update(app: AppHandle) -> updater::Result<()> {
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