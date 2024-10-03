use tauri::{
  plugin::{Builder as TauriBuilder, TauriPlugin},
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

pub struct Builder;

impl Builder {
  pub fn new() -> Self {
    Self
  }

  pub fn build<R: Runtime>(self) -> TauriPlugin<R> {
    TauriBuilder::new("updater")
      .invoke_handler(tauri::generate_handler![check_update])
      .build()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  TauriBuilder::new("updater")
    .build()
}
