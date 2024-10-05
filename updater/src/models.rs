use serde::{Deserialize, Serialize};
use tauri::{plugin::PluginHandle, Runtime};

use crate::error::BrowserOpenError;


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
  pub name: String,
  pub browser_download_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Release {
  pub tag_name: String,
  pub assets: Vec<Asset>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Update<R: Runtime> {
  pub download: String,
  pub(crate) handle: PluginHandle<R>
}

impl Update {
  pub async fn download_and_install<C: FnMut(usize, Option<u64>), D: FnOnce()>(&self, _on_chunk: C, on_download_finish: D) -> crate::Result<()> {
    on_download_finish();

    self.handle.
      run_mobile_plugin("open", self.download)
    //webbrowser::open(&self.download).map_err(|_| crate::Error::BrowserOpenError(BrowserOpenError))
  }
}