use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

mod mobile;

mod error;
mod models;

pub use error::{Error, Result};

use mobile::Updater;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the updater APIs.
pub trait UpdaterExt<R: Runtime> {
  fn updater(&self) -> Result<Updater<R>>;
}

impl<R: Runtime, T: Manager<R>> UpdaterExt<R> for T {
  fn updater(&self) -> Result<Updater<R>> {
    mobile::init(self.app_handle())
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("updater")
    .build()
}
